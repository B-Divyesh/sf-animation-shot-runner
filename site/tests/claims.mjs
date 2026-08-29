import assert from 'node:assert/strict';
import {spawn, spawnSync} from 'node:child_process';
import {once} from 'node:events';
import {existsSync, mkdtempSync, readFileSync, rmSync, statSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join, resolve} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';
import {chromium} from 'playwright';

const requested = process.argv.includes('--grep') ? process.argv[process.argv.indexOf('--grep') + 1] : '';
const selected = id => !requested || requested === id;
let claimsRun = 0;

async function claim(id, test) {
  if (!selected(id)) return;
  claimsRun += 1;
  await test();
}

function command(name, args, options = {}) {
  const result = spawnSync(name, args, {encoding: 'utf8', maxBuffer: 20 * 1024 * 1024, ...options});
  assert.equal(result.status, 0, `${name} ${args.join(' ')} failed:\n${result.stdout}\n${result.stderr}`);
  return result;
}

function cargo(testName) {
  command('cargo', ['test', '--locked', testName]);
}

async function withPreview(port, test) {
  const externalOrigin = process.env.TEST_ORIGIN?.replace(/\/$/, '');
  const origin = externalOrigin || `http://127.0.0.1:${port}`;
  const vite = externalOrigin ? null : spawn(process.execPath, [resolve('node_modules/vite/bin/vite.js'), 'preview', '--config', 'site/vite.config.js', '--host', '127.0.0.1', '--port', String(port), '--strictPort'], {stdio: 'pipe'});
  let browser;
  try {
    let ready = false;
    for (let attempt = 0; attempt < 60; attempt += 1) {
      try { if ((await fetch(origin)).ok) { ready = true; break; } } catch {}
      await delay(100);
    }
    assert.equal(ready, true, `preview server did not start on ${origin}`);
    browser = await chromium.launch({headless: true});
    await test({browser, origin});
  } finally {
    await browser?.close();
    vite?.kill('SIGTERM');
    if (vite) await once(vite, 'exit').catch(() => {});
  }
}

async function browserDemo() {
  await withPreview(4175, async ({browser, origin}) => {
    const context = await browser.newContext();
    const requests = [];
    context.on('request', request => requests.push(request.url()));
    const page = await context.newPage();
    await page.goto(origin, {waitUntil: 'networkidle'});
    await page.evaluate(() => localStorage.setItem('animation-shot-runner:real-sentinel', 'keep'));
    await page.goto(`${origin}/?demo=1`, {waitUntil: 'networkidle'});
    await page.waitForURL('**/demo/?demo=1');
    await page.getByText('Demo — sample data, nothing is saved', {exact: false}).first().waitFor();
    assert.equal(await page.locator('input[type="password"], [name*="email" i]').count(), 0, 'sample page asks for no account');
    assert.equal(await page.evaluate(() => localStorage.getItem('demo:animation-shot-runner:opened')), 'true');
    assert.equal(await page.evaluate(() => localStorage.getItem('sb_license:animation-shot-runner')), null);
    await page.getByRole('button', {name: 'Reset demo'}).click();
    await page.getByText('Sample view reset.', {exact: false}).first().waitFor();
    await page.evaluate(() => localStorage.setItem('demo:animation-shot-runner:extra', 'discard'));
    await page.getByRole('link', {name: 'Start for real'}).click();
    await page.waitForURL(`${origin}/`);
    const storage = await page.evaluate(() => Object.fromEntries(Object.entries(localStorage)));
    assert.equal(storage['animation-shot-runner:real-sentinel'], 'keep', 'real storage survives demo mode');
    assert.equal(Object.keys(storage).some(key => key.startsWith('demo:animation-shot-runner:')), false, 'Start for real discards every demo key');
    assert.ok(requests.every(url => new URL(url).origin === origin), `unexpected request: ${requests.join(', ')}`);
  });
}

async function offlineOpenedPages() {
  await withPreview(4177, async ({browser, origin}) => {
    const context = await browser.newContext();
    const page = await context.newPage();
    await page.goto(`${origin}/demo/?demo=1`, {waitUntil: 'networkidle'});
    await page.evaluate(() => navigator.serviceWorker.ready);
    await page.reload({waitUntil: 'networkidle'});
    assert.equal(await page.evaluate(() => Boolean(navigator.serviceWorker.controller)), true, 'service worker controls the opened page');
    await context.setOffline(true);
    await page.reload({waitUntil: 'domcontentloaded'});
    assert.match(await page.locator('h1').textContent(), /See five sample\s*previews run/, 'the opened demo remains available offline');
    await page.locator('.first-recording img').waitFor();
    assert.ok(await page.locator('.first-recording img').evaluate(image => image.complete && image.naturalWidth > 0), 'the opened sample contact sheet remains available offline');
  });
}

function installFromCleanMachine() {
  const html = readFileSync('site/index.html', 'utf8');
  const displayed = html.match(/<div class="command"[^>]*><code>(cargo install[^<]+)<\/code>/)?.[1];
  assert.ok(displayed, 'home page exposes an install command');
  const parts = displayed.trim().split(/\s+/);
  assert.equal(parts.shift(), 'cargo');
  const installRoot = mkdtempSync(join(tmpdir(), 'shot-runner-claim-install-'));
  const cargoHome = join(installRoot, 'cargo-home');
  let demoDirectory;
  try {
    command('cargo', parts, {
      cwd: installRoot,
      env: {...process.env, CARGO_HOME: cargoHome, CARGO_TARGET_DIR: join(installRoot, 'target')},
      timeout: 5 * 60 * 1000,
    });
    const binary = join(cargoHome, 'bin', 'shot-runner');
    assert.equal(existsSync(binary), true, 'install command creates shot-runner');
    assert.match(command(binary, ['--version'], {cwd: installRoot}).stdout, /^shot-runner 0\.1\.0/m);
    const demo = JSON.parse(command(binary, ['--json', 'demo'], {cwd: installRoot}).stdout);
    assert.equal(demo.rendered, 5);
    demoDirectory = demo.directory;
  } finally {
    if (demoDirectory) rmSync(demoDirectory, {recursive: true, force: true});
    rmSync(installRoot, {recursive: true, force: true});
  }
}

function buildOutput() {
  rmSync('dist/site', {recursive: true, force: true});
  command('npm', ['run', 'build']);
  for (const path of ['target/release/shot-runner', 'dist/site/index.html', 'dist/site/demo/index.html', 'dist/site/privacy/index.html', 'dist/site/terms/index.html', 'dist/site/404.html', 'dist/site/staticwebapp.config.json']) {
    assert.equal(existsSync(path), true, `build output is missing ${path}`);
  }
}

function packageArtifact() {
  command('npm', ['run', 'pack:cli']);
  const artifact = 'target/package/animation-shot-runner-0.1.0.crate';
  assert.equal(existsSync(artifact), true, 'cargo package artifact is missing');
  assert.ok(statSync(artifact).size > 0, 'cargo package artifact is empty');
}

await claim('@claim:demo-five-shot', () => cargo('demo_renders_five_bundled_shots_in_a_new_temp_folder'));
await claim('@claim:demo-cache-and-receipt', () => cargo('demo_receipt_detects_a_tampered_sample_frame'));
await claim('@claim:demo-project-isolation', () => cargo('demo_leaves_the_callers_project_files_unchanged'));
await claim('@claim:review-before-run', () => cargo('requires_confirmation_before_execution'));
await claim('@claim:exact-plan-command', () => cargo('planned_argv_is_the_complete_argv_recorded_after_execution'));
await claim('@claim:run-output-set', () => cargo('run_writes_copied_frames_contact_sheet_and_receipt'));
await claim('@claim:unchanged-run-cache', () => cargo('second_unchanged_run_reuses_local_cache'));
await claim('@claim:receipt-metadata', () => cargo('receipt_records_verified_hashes_fps_and_colorspace'));
await claim('@claim:direct-command-expansion', () => cargo('direct_command_expands_every_placeholder_without_shell_interpretation'));
await claim('@claim:relative-paths-and-exit-codes', () => cargo('documented_relative_paths_cache_and_exit_codes_hold'));
await claim('@claim:isolated-browser-demo', browserDemo);
await claim('@claim:offline-opened-pages', offlineOpenedPages);
await claim('@claim:install-from-clean-machine', installFromCleanMachine);
await claim('@claim:build-output', buildOutput);
await claim('@claim:package-artifact', packageArtifact);
await claim('@claim:mit-license', () => {
  const license = readFileSync('LICENSE', 'utf8');
  assert.match(license, /Permission is hereby granted, free of charge/, 'the distributed license contains the MIT grant');
});
await claim('@claim:renderer-dependencies', () => {
  cargo('native_contact_sheet_runs_without_ffmpeg_on_path');
  const packed = command('cargo', ['package', '--manifest-path', 'crates/shot-runner/Cargo.toml', '--allow-dirty', '--list']);
  assert.doesNotMatch(packed.stdout, /(^|\/)ffmpeg(\.|$)|(^|\/)blender(\.|$)/im, 'the crate ships no renderer binary');
});

assert.ok(claimsRun > 0, `unknown claim test: ${requested}`);
console.log(requested ? `claim test passed: ${requested}` : `all ${claimsRun} claim tests passed`);
