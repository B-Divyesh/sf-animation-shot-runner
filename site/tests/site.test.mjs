import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';

const pages = ['index.html', 'privacy/index.html', 'terms/index.html'];
for (const page of pages) {
  const html = readFileSync(new URL(`../${page}`, import.meta.url), 'utf8');
  assert.match(html, /<html lang="en">/, `${page} needs lang`);
  assert.equal((html.match(/<h1[ >]/g) || []).length, 1, `${page} needs exactly one h1`);
  assert.equal((html.match(/<main[ >]/g) || []).length, 1, `${page} needs main`);
  assert.match(html, /<title>[^<]+<\/title>/, `${page} needs title`);
}
const home = readFileSync(new URL('../index.html', import.meta.url), 'utf8');
assert.equal((home.match(/<img /g) || []).length, (home.match(/<img [^>]*alt=/g) || []).length, 'every image needs alt');
const script = readFileSync(new URL('../src/main.js', import.meta.url), 'utf8');
assert.match(script, /sb_license:\$\{PRODUCT\}/, 'license storage key contract');
assert.match(script, /history\.replaceState/, 'license must be stripped from URL');
assert.match(script, /86_400_000/, 'license verification must be cached for one day');
const worker = readFileSync(new URL('../public/sw.js', import.meta.url), 'utf8');
assert.match(worker, /url\.origin !== self\.location\.origin/, 'service worker caches same-origin assets only');
assert.match(worker, /license\|token\|entitlement/i, 'service worker rejects license-bearing URLs');
assert.match(worker, /url\.pathname\.includes\('\/verify'\)/, 'service worker rejects entitlement verification requests');
assert.match(worker, /shot-runner-v3/, 'service worker cache version advances with its shell');
const staticPolicy = JSON.parse(readFileSync(new URL('../public/staticwebapp.config.json', import.meta.url), 'utf8'));
const headers = staticPolicy.globalHeaders;
assert.match(headers['Content-Security-Policy'], /default-src 'self'/, 'deployment needs a self-only CSP baseline');
assert.match(headers['Content-Security-Policy'], /connect-src 'self' https:\/\/api\.sociobot\.in/, 'CSP permits the optional license verification only');
assert.match(headers['Content-Security-Policy'], /frame-ancestors 'none'/, 'CSP prevents framing');
assert.equal(headers['X-Frame-Options'], 'DENY', 'deployment has legacy frame protection');
assert.equal(headers['Cross-Origin-Opener-Policy'], 'same-origin', 'deployment isolates the browsing context');
assert.match(headers['Permissions-Policy'], /camera=\(\)/, 'deployment disables unused device permissions');
const immutableRoutes = staticPolicy.routes.filter(route => route.headers?.['Cache-Control'] === 'public, max-age=31536000, immutable');
assert.ok(immutableRoutes.some(route => route.route === '/assets/*'), 'hashed build assets are immutable');
assert.ok(immutableRoutes.some(route => /-[a-f0-9]{8}\.(webp|woff2)$/.test(route.route)), 'public immutable assets use content-named URLs');
assert.equal(staticPolicy.routes.find(route => route.route === '/sw.js')?.headers?.['Cache-Control'], 'no-cache', 'service worker remains updateable');
console.log('site contract checks passed');
