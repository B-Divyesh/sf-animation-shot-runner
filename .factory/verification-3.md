# Independent verification 3 — PASS

**Verified:** 2026-08-27  
**Candidate commit:** `e2d992be9b401f5ebdf848284ee9613e2e0259db` (`test: verify PWA cache policy against live site`)  
**Live URL:** <https://animation-shot-runner.sociobot.in/>

## Verdict

**PASS — release candidate accepted.** A fresh clone at the exact candidate passed its available test, format, lint, production-build, accessibility, package, and clean-consumer checks. The live documentation/PWA is byte-identical to the candidate's built public output for every served asset checked. The local CLI's executable is not deployable to the documentation URL, so live identity can establish the site/PWA only; the separately packaged binary was tested from the candidate's Cargo package.

The previously blocking command-review defect is fixed: both human and JSON `plan` output disclose every unexpanded manifest token and the complete expanded argv that the CLI passes to the renderer. The packed CLI recorded the same argv in its receipt.

## Clean-checkout quality gates

| Check | Result | Fresh evidence |
| --- | --- | --- |
| Clean clone and identity | PASS | Cloned `origin/main` with `git clone --no-local`; HEAD was exactly `e2d992be9b401f5ebdf848284ee9613e2e0259db` and clean before test output. |
| Install | PASS | `npm ci`: 20 packages installed; 0 vulnerabilities reported. |
| Unit/integration/site tests | PASS | `npm test`: 6 Rust unit tests, 3 CLI integration tests, site contracts, and PWA Cache Storage regression all passed. |
| Rust format/lint | PASS | `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings` passed. No separate TypeScript or lint script is defined by the repository. |
| Exact production build | PASS | `npm run build` completed: locked optimized Rust binary plus `dist/site/`. |
| Accessibility | PASS | `npm run test:a11y` and the same test against the live URL each returned 40 axe passes, 0 violations, 0 serious/critical. |
| Publishable artifact | PASS | `npm run pack:cli` ran `cargo package --allow-dirty`, packaged 8 files / 57.3 KiB (15.4 KiB compressed), and Cargo verified it. No publishing was attempted. |

## Packaged CLI: independent consumer exercise

Only the package at `target/package/animation-shot-runner-0.1.0` was installed into a new temporary consumer with `cargo install --path ... --root <consumer>/install --locked`. Its one `shot-runner` binary exposed useful `init`, `plan`, `run`, and `verify` help plus global `--json`.

A five-shot manifest used `cp` as a representative allowlisted local renderer, with normal 24 fps, 23.976 fps, the boundary positive value `0.001`, and 60/120 fps; colorspaces included sRGB, ACEScg, Display P3, and Rec.709.

- `plan --json --cache-dir fresh-cache` produced five items. Each included the entire token vector `['cp', 'frame.png', '{frames}/frame-0001.png']`, exact absolute argv, source hash context, cache directory, FPS, and colorspace.
- `run --allow-command cp --yes --cache-dir fresh-cache --json` rendered 5/5 shots, each with PNG frame, contact sheet, and receipt. Repeating the exact run yielded `rendered: 0, cache_hits: 5`.
- `verify` accepted a receipt. Replacing one rendered frame made `verify` fail with exit 5 and `hash mismatch`; a selected `run --shot sq003` restored the cached output and verification passed.
- A run without `--yes` exited 3; a nonmatching `--allow-command blender` exited 3; unknown `--shot` and invalid `fps: 0` plan input each exited 2 with actionable JSON errors.

This satisfies the brief's five-named-shot, content-cache, contact-sheet, receipt, allowlist, confirmation, repeatability, boundary, invalid-input, and recovery paths without a browser UI or network renderer.

## Site/PWA, privacy, deployment, and browser checks

- SHA-256 comparisons between `dist/site/` and live responses matched for `index.html`, `sw.js`, generated JS/CSS, both self-hosted WOFF2 fonts, both proof images, and `/privacy/` and `/terms/`. The candidate's `shot-runner-v3` worker is therefore live.
- Desktop (1440 px) and mobile (390 px) browser runs had no console or page errors and no horizontal overflow. The browser manifest desk planned the five-shot normal sample, rejected malformed JSON with recovery guidance, and planned successfully after correction.
- Initial loads made no outbound request; requests were same-origin only. The optional license flow makes a Sociobot verification request only after a token is supplied. Source inspection found no CLI networking, telemetry, analytics, CDN fonts/scripts, `eval`, or media upload.
- Keyboard testing found a visible `rgb(18, 18, 16) solid 3px` focus outline on the reached interactive control at both sizes. The manifest control remains operable by keyboard as a native textarea/button. Under reduced motion, computed transition and animation durations were both `0s`.
- Live PWA: after reload it had a controlling worker; `registration.update()` completed; a 390 px offline reload returned the cached title/page and displayed the offline notice, with no page/console errors. The shipped PWA regression also confirmed Cache Storage has no license/token/entitlement/`/verify` URL or Sociobot entitlement response.
- `/privacy/` and `/terms/` returned 200. Live headers include HSTS, `nosniff`, `strict-origin-when-cross-origin`, self-only CSP with the explicit Sociobot license connection, `frame-ancestors 'none'`, `X-Frame-Options: DENY`, `Cross-Origin-Opener-Policy: same-origin`, and restrictive Permissions-Policy. `sw.js` is `no-cache`; Vite hashed JS/CSS have one-year immutable caching.
- Bundle budgets pass independently: initial JS 6,478 B, CSS 10,432 B, total WOFF2 35,740 B, mobile proof 29,572 B, desktop proof 148,544 B. All are below their applicable limits.

Lighthouse 13.4.1 was attempted with Playwright's Chromium explicitly configured, but Chrome crashed its tab before producing a report. This is a QA-environment limitation, not a product defect: the live axe, responsive, offline, console, header, and bundle checks above completed successfully.

## Defects

No release-blocking, high, medium, or low product defects found in this verification.

## Release notes

The production site is confirmed to be this candidate's static documentation/PWA output. The CLI is deliberately local and has no server build identity endpoint; release automation should distribute the Cargo package/binary built from this same commit. The ready-to-publish command remains:

```sh
npm run pack:cli
```
