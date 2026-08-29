import {spawn} from 'node:child_process';
import {once} from 'node:events';
import {mkdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';
import {chromium} from 'playwright';

const port = 4176;
const localOrigin = `http://127.0.0.1:${port}`;
const origin = (process.env.CAPTURE_ORIGIN || localOrigin).replace(/\/$/, '');
const prefix = process.env.CAPTURE_PREFIX || '';
const vite = process.env.CAPTURE_ORIGIN ? null : spawn(process.execPath, [resolve('node_modules/vite/bin/vite.js'), 'preview', '--config', 'site/vite.config.js', '--host', '127.0.0.1', '--port', String(port), '--strictPort'], {stdio: 'pipe'});
try {
  let ready = false;
  for (let attempt = 0; attempt < 60; attempt += 1) { try { if ((await fetch(origin)).ok) { ready = true; break; } } catch {} await delay(100); }
  if (!ready) throw new Error(`capture target did not start: ${origin}`);
  const browser = await chromium.launch({headless: true});
  await mkdir('.factory/evidence', {recursive: true});
  for (const [name, path, viewport] of [
    ['home-390.png', '/', {width: 390, height: 844}],
    ['demo-390.png', '/demo/?demo=1', {width: 390, height: 844}],
    ['home-1440.png', '/', {width: 1440, height: 900}],
  ]) {
    const page = await browser.newPage({viewport});
    const errors = [];
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    await page.goto(`${origin}${path}`, {waitUntil: 'networkidle'});
    if (errors.length) throw new Error(`${path} console errors: ${errors.join('; ')}`);
    await page.screenshot({path: `.factory/evidence/${prefix}${name}`, fullPage: false});
    await page.close();
  }
  await browser.close();
  console.log('captured mobile and desktop evidence without console errors');
} finally { vite?.kill('SIGTERM'); if (vite) await once(vite, 'exit').catch(() => {}); }
