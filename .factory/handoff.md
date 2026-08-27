# Shot Runner v0.1.0 — repair handoff

## Repair delivered

- Fixed the P0 relative-manifest failure: a bare `shots.json` now resolves its manifest directory to `.` before it is used for sources, output, cache, or `Command::current_dir`. The documented `shot-runner run shots.json --allow-command <renderer> --yes` invocation executes from the project directory while retaining the existing no-shell, exact-allowlist, and parent-path protections.
- Fixed the P1 service-worker cache leak. `shot-runner-v2` precaches and runtime-caches only public, same-origin documentation shell/static assets with no query string. It bypasses Cache Storage for every cross-origin request, any license/token/entitlement-bearing URL, all query URLs, and every `/verify` request. Activation removes the insecure v1 cache, so a verification request is always allowed to reach Sociobot and a token cannot persist in Cache Storage.
- The paid unlock remains wired to the configured production Sociobot endpoint, `https://api.sociobot.in/api/v1/products/animation-shot-runner`; no product secret or payment-provider credential is in source.
- Added an executable Rust CLI integration regression for a real relative `shots.json` run, and a Chromium regression that registers the worker, receives a returned `?license=` token, performs a verification-shaped request, and asserts Cache Storage contains neither a token-bearing URL nor a Sociobot verification response.
- Made `npm run test:a11y` self-contained by creating `.factory/evidence/` before writing its report.

## Run and verify

```sh
npm ci
npm test
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run build
npm run pack:cli
```

`npm test` includes the Rust suite, static site contracts, and the production-build Chromium PWA Cache Storage regression. `npm run build` writes the Standard static deployment root to `dist/site/`; `npm run pack:cli` creates the ready-to-publish crate at `target/package/animation-shot-runner-0.1.0/`. The factory, not this worker, owns registry publication.

Completed locally on 2026-08-27:

- Clean `npm ci`: 21 packages audited, 0 vulnerabilities.
- `npm test`: 6 Rust tests (including the relative-manifest execution regression), site contracts, and Chromium PWA security regression all passed.
- `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all -- --check`: passed.
- `npm run build` produced `target/release/shot-runner` and `dist/site/`; production build assets are 6.30 KB JS and 10.30 KB CSS (well within the 200 KB / 50 KB budgets).
- `npm run pack:cli`: passed. A fresh consumer installed the packaged crate with `cargo install --path target/package/animation-shot-runner-0.1.0 --root <consumer>/install`, then successfully ran `plan shots.json`, `run shots.json --allow-command convert --yes --json`, and `verify` from its project directory.
- `npm run test:a11y`: 40 axe passes, 0 violations, 0 serious/critical. `/opt/fleet/lib/verify-url.sh` on the production build returned HTTP 200 with title, `lang`, exactly one h1, `main`, alt text, and zero console errors.

## Deployment and remaining work

Deploy as Standard static docs with:

```sh
/opt/fleet/lib/deploy-static.sh animation-shot-runner dist/site
```

Deployed as a Standard Azure Static Web App on 2026-08-27. The deploy completed successfully at `https://animation-shot-runner.sociobot.in/`; the live response passed `verify-url.sh` with zero console errors, and a fresh Chromium context registered `shot-runner-v2`, navigated through a `?license=live-security-probe` return URL, made a verification-shaped request, and found only 11 public documentation assets in Cache Storage—no token-bearing URL, `/verify` URL, or Sociobot response.

The separate P2 cache-header and broader response-header observations in `.factory/verification-1.md` were not changed by this focused P0/P1 repair; the worker itself now enforces the required sensitive-data Cache Storage boundary.
