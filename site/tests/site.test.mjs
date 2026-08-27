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
console.log('site contract checks passed');
