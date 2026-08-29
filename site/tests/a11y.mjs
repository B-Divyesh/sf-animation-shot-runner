import {AxeBuilder} from '@axe-core/playwright';
import {chromium} from 'playwright';
import {spawn} from 'node:child_process';
import {once} from 'node:events';
import {mkdir, writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';

const port = 4173;
const localUrl = `http://127.0.0.1:${port}/`;
const url = new URL('/', process.env.TEST_URL || localUrl).href;
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
  const reports = [];
  for (const path of ['/', '/demo/?demo=1', '/privacy/', '/terms/', '/404.html']) {
    const context = await browser.newContext({viewport: {width: 390, height: 844}});
    const page = await context.newPage();
    await page.goto(new URL(path, url).href, {waitUntil: 'networkidle'});
    reports.push({path, results: await new AxeBuilder({page}).analyze()});
    await context.close();
  }
  await mkdir('.factory/evidence', {recursive: true});
  await writeFile('.factory/evidence/axe.json', JSON.stringify(reports, null, 2));
  const violations = reports.flatMap(report => report.results.violations.map(violation => ({path: report.path, ...violation})));
  const passes = reports.reduce((total, report) => total + report.results.passes.length, 0);
  console.log(`axe: ${passes} route checks passed, ${violations.length} violations across ${reports.length} routes`);
  for (const violation of violations) console.error(`${violation.path} ${violation.impact}: ${violation.id} — ${violation.help}`);
  if (violations.length) process.exitCode = 1;
} finally {
  await browser?.close();
  vite?.kill('SIGTERM');
  await viteExited?.catch(() => {});
}
