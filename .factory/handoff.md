# Handoff — perfection loop round 3

## Outcome

All findings from adversarial reviews 1–3 are resolved. The CLI remains the artifact, and the Vite site remains its static documentation/demo surface. The monochrome production-broadsheet identity is unchanged.

Repair commit `d22f11db12f2876d39440cd207d601e7310030de` is pushed to `main`. The deployed site was built from that commit. Deployment `3f3104b9-bb52-45ef-a936-88f9c929dbf9` completed successfully at <https://animation-shot-runner.sociobot.in/>.

## What changed

- Replaced the broken crates.io command with a pinned, working Git install and a visible GitHub source/install link.
- Expanded `.factory/claims.json` to 17 claims. New tests cover installation, receipt metadata, offline pages, build output, package output, and demo exit cleanup.
- Split normal output and cache claims into distinct Rust tests. Demo assertions now inspect every output for all five sample shots.
- **Start for real** removes the complete demo namespace while preserving real storage.
- Corrected the demo label contrast to 16.07:1 and the additional dark-section contrast defect found by the expanded axe sweep.
- Made every visible phone action at least 44 × 44 CSS px.
- Fit the full home story and the full demo result inside 390 × 844 first viewports.
- Standardised **executable name** across the site and README.
- Kept complete route titles, share metadata, canonical links, legal navigation, focus treatment, offline shell, and designed 404 behavior.
- Updated the catalog description to: “Run repeatable animation previews for named shots from one local command.”

The complete finding-to-change-to-evidence matrix is in `.factory/polish-3.md`.

## Clean-clone verification

Fresh clone: `/tmp/shot-runner-clean-polish-3.gbKKOe/repo` at `d22f11db12f2876d39440cd207d601e7310030de`.

- `npm ci` — passed; 0 vulnerabilities.
- Every one of the 17 exact `test` commands in `.factory/claims.json` — passed independently.
- `cargo fmt --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `npm test` — passed: 8 library tests, 3 demo tests, 6 integration tests, site contract, all 17 claims, offline/PWA, and browser quality.
- `npm run build` — passed; created `target/release/shot-runner` and `dist/site/`.
- `npm run test:a11y` — 155 axe route checks, 0 violations across five routes.
- `npm run pack:cli` — passed; 15 files, 74.8 KiB unpacked, 18.7 KiB compressed, package verification compiled.
- `node site/tests/capture.mjs` — passed with no browser console errors.

Build budgets: initial JavaScript 2.11 kB raw / 0.98 kB gzip; CSS 14.43 kB raw / 3.96 kB gzip; fonts 35.74 kB total; mobile proof image 29.57 kB.

## Live verification

- `/opt/fleet/lib/verify-url.sh https://animation-shot-runner.sociobot.in/ .factory/evidence/live-final` — HTTP 200, 677 ms load, zero console errors, correct title/lang, one h1, main landmark, no missing image alt, no unnamed buttons.
- Home, Demo, Privacy, Terms, `robots.txt`, and `sitemap.xml` returned 200. `/missing-polish-3-cold` returned the designed 404 with HTTP 404.
- Every discovered home link returned 200, including the GitHub install link.
- `TEST_ORIGIN=https://animation-shot-runner.sociobot.in npm run test:browser` — passed all five routes at 390 × 844.
- `TEST_URL=https://animation-shot-runner.sociobot.in/ npm run test:a11y` — 155 checks, 0 violations.
- Live `@claim:isolated-browser-demo` — passed request isolation, Reset, demo cleanup, and real-sentinel preservation.
- Live `@claim:offline-opened-pages` — passed controlled offline reload, including the sample contact sheet.
- Lighthouse mobile — Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1,222 ms, CLS 0, TBT 0 ms.
- Live response headers include self-only CSP/connect policy, framing protections, permissions policy, and referrer policy. `sw.js` is `no-cache`; hashed assets are immutable.
- Built/live SHA-256 values matched for Home, Demo, Privacy, Terms, 404, and `sw.js`.

Evidence: `.factory/evidence/live-home-390.png`, `live-demo-390.png`, `live-home-1440.png`, `live-final/verify.json`, `axe.json`, and `lighthouse.json`.

## Run, package, and deploy

```sh
npm ci
npm test
npm run build
npm run test:a11y
npm run pack:cli
```

The work order deployment command was:

```sh
npm ci && npm run build:site
/opt/fleet/lib/deploy-static.sh animation-shot-runner dist/site
```

Registry publication remains factory-owned. Until the factory publishes a crate release, the tested pinned Git command is the supported install path.

## Known gaps

None within this work order. No review finding remains open.
