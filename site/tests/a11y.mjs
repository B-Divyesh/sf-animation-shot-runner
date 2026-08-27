import {AxeBuilder} from '@axe-core/playwright';
import {chromium} from 'playwright';
import {spawn} from 'node:child_process';
import {once} from 'node:events';
import {mkdir, writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';

const port = 4173;
const localUrl = `http://127.0.0.1:${port}/`;
const url = process.env.TEST_URL || localUrl;
const vite = process.env.TEST_URL ? null : spawn(process.execPath, [
  resolve('node_modules/vite/bin/vite.js'),
  'preview',
  '--config', 'site/vite.config.js',
  '--host', '127.0.0.1',
  '--port', String(port),
  '--strictPort',
], {stdio: 'ignore'});
const viteExited = vite && once(vite, 'exit');

async function waitForServer() {
  for (let attempt = 0; attempt < 50; attempt += 1) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {}
    await delay(100);
  }
  throw new Error(`Axe target did not start: ${url}`);
}

let browser;
try {
  await waitForServer();
  browser = await chromium.launch({headless: true});
  const context = await browser.newContext({viewport: {width: 390, height: 844}});
  const page = await context.newPage();
  await page.goto(url, {waitUntil: 'networkidle'});
  const results = await new AxeBuilder({page}).analyze();
  await mkdir('.factory/evidence', {recursive: true});
  await writeFile('.factory/evidence/axe.json', JSON.stringify(results, null, 2));
  const serious = results.violations.filter(item => ['serious', 'critical'].includes(item.impact));
  console.log(`axe: ${results.passes.length} passes, ${results.violations.length} violations, ${serious.length} serious/critical`);
  for (const violation of serious) console.error(`${violation.impact}: ${violation.id} — ${violation.help}`);
  if (serious.length) process.exitCode = 1;
} finally {
  await browser?.close();
  vite?.kill('SIGTERM');
  await viteExited?.catch(() => {});
}
