import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';

const pages = ['index.html', 'demo/index.html', 'privacy/index.html', 'terms/index.html', '404.html'];
for (const page of pages) {
  const html = readFileSync(new URL(`../${page}`, import.meta.url), 'utf8');
  assert.match(html, /<html lang="en">/, `${page} needs lang`);
  assert.equal((html.match(/<h1[ >]/g) || []).length, 1, `${page} needs exactly one h1`);
  assert.equal((html.match(/<main[ >]/g) || []).length, 1, `${page} needs main`);
  assert.match(html, /<title>[^<]+<\/title>/, `${page} needs title`);
  assert.match(html, /rel="canonical"/, `${page} needs canonical URL`);
  assert.match(html, /property="og:title"/, `${page} needs Open Graph title`);
  assert.match(html, /name="twitter:card"/, `${page} needs Twitter card`);
  assert.match(html, /name="twitter:title"/, `${page} needs Twitter title`);
  assert.match(html, /name="twitter:description"/, `${page} needs Twitter description`);
  assert.match(html, /name="twitter:image"/, `${page} needs Twitter image`);
  assert.match(html, /apple-touch-icon/, `${page} needs touch icon`);
}
const home = readFileSync(new URL('../index.html', import.meta.url), 'utf8');
assert.equal((home.match(/<img /g) || []).length, (home.match(/<img [^>]*alt=/g) || []).length, 'every image needs alt');
assert.match(home, /cargo install --git https:\/\/github\.com\/B-Divyesh\/sf-animation-shot-runner\.git --rev [a-f0-9]{40} --locked animation-shot-runner/, 'home needs a pinned Git install command');
assert.match(home, /Open source and install on GitHub/, 'home needs a visible source and install link');
assert.doesNotMatch(home, /approve the program name/, 'approval terminology stays consistent');
assert.match(home, />Shot Runner<\/a>/, 'the visible wordmark names the product');
assert.doesNotMatch(home, /SR \/ 01|VOL\. 01/, 'the home removes decorative issue labels');
assert.match(home, /JSON receipt that records output hashes, frame rate, and colour space/, 'the first home receipt explains what it records');
const demo = readFileSync(new URL('../demo/index.html', import.meta.url), 'utf8');
assert.match(demo, /JSON receipt with output hashes, frame rate, and colour space/, 'the first demo receipt explains what it records');
for (const page of pages) {
  const html = readFileSync(new URL(`../${page}`, import.meta.url), 'utf8');
  assert.match(html, />Shot Runner<\/a>/, `${page} has a visible product wordmark`);
  assert.doesNotMatch(html, /SR \/ 01|VOL\. 01/, `${page} has no decorative issue labels`);
}
const script = readFileSync(new URL('../src/main.js', import.meta.url), 'utf8');
assert.match(script, /demo:animation-shot-runner:opened/, 'demo storage must have its own namespace');
assert.match(script, /location\.replace\('\/demo\/\?demo=1'\)/, 'root demo query opens the real demo route');
assert.match(script, /startsWith\(DEMO_PREFIX\)/, 'leaving the demo removes its full namespace');
assert.match(script, /history\.scrollRestoration = 'manual'/, 'document routes own scroll restoration');
assert.match(script, /ROUTE_TRANSITION_KEY/, 'document routes track new-route heading focus');
assert.match(script, /dataset\.routeAnnouncement/, 'document routes announce their new heading');
const worker = readFileSync(new URL('../public/sw.js', import.meta.url), 'utf8');
assert.match(worker, /url\.origin !== self\.location\.origin/, 'service worker caches same-origin assets only');
assert.match(worker, /license\|token\|entitlement/i, 'service worker rejects license-bearing URLs');
assert.match(worker, /url\.pathname\.includes\('\/verify'\)/, 'service worker rejects entitlement verification requests');
assert.match(worker, /shot-runner-v5/, 'service worker cache version advances with its shell');
const staticPolicy = JSON.parse(readFileSync(new URL('../public/staticwebapp.config.json', import.meta.url), 'utf8'));
assert.equal(staticPolicy.navigationFallback, undefined, 'static routes must not hide unknown URLs behind the landing page');
const headers = staticPolicy.globalHeaders;
assert.match(headers['Content-Security-Policy'], /default-src 'self'/, 'deployment needs a self-only CSP baseline');
assert.match(headers['Content-Security-Policy'], /connect-src 'self';/, 'CSP allows only same-origin connections');
assert.match(headers['Content-Security-Policy'], /frame-ancestors 'none'/, 'CSP prevents framing');
assert.equal(headers['X-Frame-Options'], 'DENY', 'deployment has legacy frame protection');
assert.equal(headers['Cross-Origin-Opener-Policy'], 'same-origin', 'deployment isolates the browsing context');
assert.match(headers['Permissions-Policy'], /camera=\(\)/, 'deployment disables unused device permissions');
const immutableRoutes = staticPolicy.routes.filter(route => route.headers?.['Cache-Control'] === 'public, max-age=31536000, immutable');
assert.ok(immutableRoutes.some(route => route.route === '/assets/*'), 'hashed build assets are immutable');
assert.ok(immutableRoutes.some(route => /-[a-f0-9]{8}\.(webp|woff2)$/.test(route.route)), 'public immutable assets use content-named URLs');
assert.equal(staticPolicy.routes.find(route => route.route === '/sw.js')?.headers?.['Cache-Control'], 'no-cache', 'service worker remains updateable');
assert.equal(staticPolicy.responseOverrides['404'].statusCode, 404, 'unknown routes must return a real 404');
const claims = JSON.parse(readFileSync(new URL('../../.factory/claims.json', import.meta.url), 'utf8'));
assert.equal(new Set(claims.map(claim => claim.id)).size, claims.length, 'claim IDs must be unique');
const claimTests = readFileSync(new URL('claims.mjs', import.meta.url), 'utf8');
for (const claim of claims) {
  const tag = `@claim:${claim.id}`;
  assert.equal(claim.test, `npm run test:claims -- --grep ${tag}`, `${claim.id} exposes its exact tagged test`);
  assert.equal(claimTests.split(`'${tag}'`).length - 1, 1, `${claim.id} has exactly one tagged test implementation`);
}
console.log('site contract checks passed');
