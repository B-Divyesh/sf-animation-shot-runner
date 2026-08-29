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
if (selected('@claim:demo-project-isolation')) cargo('demo_leaves_the_callers_project_files_unchanged');
if (selected('@claim:review-before-run')) cargo('requires_confirmation_before_execution');
if (selected('@claim:exact-plan-command')) cargo('planned_argv_is_the_complete_argv_recorded_after_execution');
if (selected('@claim:run-output-set') || selected('@claim:unchanged-run-cache')) cargo('runs_caches_and_verifies_a_real_preview');
if (selected('@claim:direct-command-expansion')) cargo('direct_command_expands_every_placeholder_without_shell_interpretation');
if (selected('@claim:relative-paths-and-exit-codes')) cargo('documented_relative_paths_cache_and_exit_codes_hold');
if (selected('@claim:isolated-browser-demo')) await browserDemo();
if (selected('@claim:mit-license')) {
  const license = readFileSync('LICENSE', 'utf8');
  assert.match(license, /Permission is hereby granted, free of charge/, 'the distributed license must contain the MIT grant');
}
if (selected('@claim:renderer-dependencies')) {
  cargo('native_contact_sheet_runs_without_ffmpeg_on_path');
  const packed = spawnSync('cargo', ['package', '--manifest-path', 'crates/shot-runner/Cargo.toml', '--allow-dirty', '--list'], {encoding: 'utf8'});
  assert.equal(packed.status, 0, packed.stderr);
  assert.doesNotMatch(packed.stdout, /(^|\/)ffmpeg(\.|$)|(^|\/)blender(\.|$)/im, 'the crate ships no renderer binary');
}
console.log(requested ? `claim test passed: ${requested}` : 'all claim tests passed');
