import assert from 'node:assert/strict';
import {spawn} from 'node:child_process';
import {once} from 'node:events';
import {resolve} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';
import {chromium} from 'playwright';

const port = 4174;
const origin = `http://127.0.0.1:${port}`;
const vite = spawn(process.execPath, [resolve('node_modules/vite/bin/vite.js'), 'preview', '--config', 'site/vite.config.js', '--host', '127.0.0.1', '--port', String(port), '--strictPort'], {stdio: 'pipe'});
const viteExited = once(vite, 'exit');
async function waitForServer() {
  let lastError = '';
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try { const response = await fetch(origin); if (response.ok) return; } catch (error) { lastError = String(error); }
    await delay(100);
  }
  throw new Error(`Vite preview did not start: ${lastError}`);
}

let browser;
try {
  await waitForServer();
  browser = await chromium.launch({headless: true});
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(`${origin}/demo/?demo=1`, {waitUntil: 'networkidle'});
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload({waitUntil: 'networkidle'});
  assert.equal(await page.evaluate(() => Boolean(navigator.serviceWorker.controller)), true, 'service worker controls the demo page');
  await context.setOffline(true);
  await page.reload({waitUntil: 'domcontentloaded'});
  assert.match(await page.locator('h1').textContent(), /Run sample previews/, 'opened demo is available offline');
  await context.setOffline(false);
  assert.equal((await fetch(`${origin}/`)).ok, true, 'preview server remains reachable after service-worker reload');
  console.log('PWA demo/offline regression passed');
} finally {
  await browser?.close();
  vite.kill('SIGTERM');
  await viteExited.catch(() => {});
}
