import assert from 'node:assert/strict';
import {spawn, spawnSync} from 'node:child_process';
import {once} from 'node:events';
import {resolve} from 'node:path';
import {readFileSync} from 'node:fs';
import {setTimeout as delay} from 'node:timers/promises';
import {chromium} from 'playwright';

const requested = process.argv.includes('--grep') ? process.argv[process.argv.indexOf('--grep') + 1] : '';
const selected = id => !requested || requested.includes(id);
function cargo(testName) {
  const result = spawnSync('cargo', ['test', '--locked', testName], {encoding: 'utf8'});
  assert.equal(result.status, 0, `${testName} failed:\n${result.stdout}\n${result.stderr}`);
}
async function browserDemo() {
  const port = 4175;
  const origin = `http://127.0.0.1:${port}`;
  const vite = spawn(process.execPath, [resolve('node_modules/vite/bin/vite.js'), 'preview', '--config', 'site/vite.config.js', '--host', '127.0.0.1', '--port', String(port), '--strictPort'], {stdio: 'pipe'});
  try {
    for (let attempt = 0; attempt < 60; attempt += 1) { try { if ((await fetch(origin)).ok) break; } catch {} await delay(100); }
    const browser = await chromium.launch({headless: true});
    const context = await browser.newContext();
    const requests = [];
    context.on('request', request => requests.push(request.url()));
    const page = await context.newPage();
    await page.goto(`${origin}/?demo=1`, {waitUntil: 'networkidle'});
    await page.waitForURL('**/demo/?demo=1');
    await expectText(page, 'Demo — sample data, nothing is saved');
    assert.equal(await page.locator('input[type="password"], [name*="email" i]').count(), 0, 'sample page does not ask for an account');
    assert.equal(await page.evaluate(() => localStorage.getItem('demo:animation-shot-runner:opened')), 'true');
    assert.equal(await page.evaluate(() => localStorage.getItem('sb_license:animation-shot-runner')), null);
    await page.getByRole('button', {name: 'Reset demo'}).click();
    await expectText(page, 'Sample view reset.');
    assert.ok(requests.every(url => new URL(url).origin === origin), `unexpected request: ${requests.join(', ')}`);
    await browser.close();
  } finally { vite.kill('SIGTERM'); await once(vite, 'exit').catch(() => {}); }
}
async function expectText(page, value) { await page.getByText(value, {exact: false}).first().waitFor(); }

if (selected('@claim:demo-five-shot')) cargo('demo_renders_five_bundled_shots_in_a_new_temp_folder');
if (selected('@claim:demo-cache-and-receipt')) cargo('demo_receipt_detects_a_tampered_sample_frame');
if (selected('@claim:review-before-run')) cargo('requires_confirmation_before_execution');
if (selected('@claim:isolated-browser-demo')) await browserDemo();
if (selected('@claim:local-files-and-license')) {
  cargo('demo_renders_five_bundled_shots_in_a_new_temp_folder');
  const license = readFileSync('LICENSE', 'utf8');
  const source = readFileSync('crates/shot-runner/src/lib.rs', 'utf8');
  assert.match(license, /Permission is hereby granted, free of charge/, 'the distributed license must contain the MIT grant');
  assert.match(source, /let output_root = safe_join/, 'run output must resolve under the manifest directory');
  assert.match(source, /fs::write\(&receipt_path/, 'a run must write a local receipt');
}
console.log(requested ? `claim test passed: ${requested}` : 'all claim tests passed');
