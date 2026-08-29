import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {once} from 'node:events';
import {resolve} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';
import {chromium} from 'playwright';

const port = 4178;
const localOrigin = `http://127.0.0.1:${port}`;
const origin = (process.env.TEST_ORIGIN || localOrigin).replace(/\/$/, '');
const vite = process.env.TEST_ORIGIN ? null : spawn(process.execPath, [
  resolve('node_modules/vite/bin/vite.js'),
  'preview',
  '--config', 'site/vite.config.js',
  '--host', '127.0.0.1',
  '--port', String(port),
  '--strictPort',
], {stdio: 'pipe'});
const viteExited = vite && once(vite, 'exit');

function rgb(value) {
  const channels = value.match(/[\d.]+/g)?.slice(0, 3).map(Number);
  assert.equal(channels?.length, 3, `could not parse colour ${value}`);
  return channels;
}

function luminance(channels) {
  const linear = channels.map(channel => {
    const value = channel / 255;
    return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2];
}

function contrast(foreground, background) {
  const values = [luminance(rgb(foreground)), luminance(rgb(background))].sort((a, b) => b - a);
  return (values[0] + 0.05) / (values[1] + 0.05);
}

async function waitForServer() {
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try { if ((await fetch(origin)).ok) return; } catch {}
    await delay(100);
  }
  throw new Error(`browser target did not start: ${origin}`);
}

let browser;
try {
  await waitForServer();
  browser = await chromium.launch({headless: true});
  for (const path of ['/', '/demo/?demo=1', '/privacy/', '/terms/', '/404.html']) {
    const context = await browser.newContext({viewport: {width: 390, height: 844}});
    const page = await context.newPage();
    const errors = [];
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('pageerror', error => errors.push(error.message));
    await page.goto(`${origin}${path}`, {waitUntil: 'networkidle'});
    assert.deepEqual(errors, [], `${path} emitted browser errors`);
    assert.equal(await page.locator('h1').count(), 1, `${path} has one h1`);
    assert.equal(await page.locator('main').count(), 1, `${path} has one main landmark`);
    assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth), true, `${path} has no horizontal overflow`);

    const undersized = await page.locator('a[href], button, input, select, textarea, [tabindex="0"]').evaluateAll(elements => elements
      .filter(element => {
        const style = getComputedStyle(element);
        const box = element.getBoundingClientRect();
        return style.visibility !== 'hidden' && style.display !== 'none' && box.width > 0 && box.height > 0;
      })
      .map(element => {
        const box = element.getBoundingClientRect();
        return {label: element.getAttribute('aria-label') || element.textContent.trim().replace(/\s+/g, ' ').slice(0, 60), width: box.width, height: box.height};
      })
      .filter(item => item.width < 43.5 || item.height < 43.5));
    assert.deepEqual(undersized, [], `${path} has targets below 44 by 44 CSS pixels`);

    await page.keyboard.press('Tab');
    assert.equal(await page.evaluate(() => document.activeElement?.classList.contains('skip')), true, `${path} focuses the skip link first`);
    const focusStyle = await page.locator('.skip').evaluate(element => getComputedStyle(element).outlineWidth);
    assert.ok(Number.parseFloat(focusStyle) >= 3, `${path} exposes a designed focus ring`);

    if (path.startsWith('/demo/')) {
      const colours = await page.locator('.first-recording .terminal-head span').first().evaluate(element => ({
        foreground: getComputedStyle(element).color,
        background: getComputedStyle(element.closest('.terminal-recording')).backgroundColor,
      }));
      assert.ok(contrast(colours.foreground, colours.background) >= 4.5, `demo terminal label contrast is below 4.5:1: ${JSON.stringify(colours)}`);
      const imageBox = await page.locator('.first-recording img').boundingBox();
      assert.ok(imageBox && imageBox.y + imageBox.height <= 844, `demo contact sheet leaves the first phone viewport: ${JSON.stringify(imageBox)}`);
    }
    if (path === '/') {
      const primaryTop = await page.getByRole('link', {name: 'Try it with sample data'}).evaluate(element => element.getBoundingClientRect().top);
      assert.ok(primaryTop < 844, `home sample action starts below the first phone viewport at ${primaryTop}px`);
      for (const locator of [page.locator('.command'), page.getByRole('link', {name: 'Open source and install on GitHub'}), page.locator('.plain-facts')]) {
        const box = await locator.boundingBox();
        assert.ok(box && box.y + box.height <= 844, `home first-screen content leaves the phone viewport: ${JSON.stringify(box)}`);
      }
    }
    await context.close();
  }
  console.log('browser quality checks passed at 390 by 844');
} finally {
  await browser?.close();
  vite?.kill('SIGTERM');
  await viteExited?.catch(() => {});
}
