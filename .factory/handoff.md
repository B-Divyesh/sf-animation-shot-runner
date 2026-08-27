# Shot Runner — repair handoff

## Release verdict: PASS

Repaired the independent QA findings from `c548c57994b887ed6c8d51310affbc6af68a4bcf`. The deployable product commit is `5a662981ed755eccfa139cb026316d465598d7a8`; the site is deployed as an Azure Static Web Apps **Standard** static site at <https://animation-shot-runner.sociobot.in/>.

## What changed

- `shot-runner plan` now exposes both the exact manifest token vector (`command`) and the exact expanded argv (`argv`) that `run` passes to `Command`. Both are produced from the same prepared-shot representation, and receipts record that exact argv.
- Added `plan --cache-dir DIRECTORY`, matching `run --cache-dir`, so an operator can review the correct `{cache}` and `{frames}` expansions. Command paths are resolved absolutely while the renderer still starts in the manifest directory.
- Rejected placeholders in executable positions, retaining an exact executable allowlist. README and browser demo now explicitly surface the full vectors and review boundary.
- Added regressions for argv/receipt parity, bare `shots.json`, and named relative manifests such as `project/shots.json`.
- Added `staticwebapp.config.json` for Azure: immutable caching for Vite hashed assets and content-named fonts/proof images; `sw.js` stays `no-cache`. The proof images and self-hosted fonts now use content-named URLs.
- Added a compatible CSP (including the optional Sociobot verification endpoint), `frame-ancestors 'none'`, `X-Frame-Options: DENY`, `Permissions-Policy`, and `Cross-Origin-Opener-Policy: same-origin`.
- Kept the worker restricted to public same-origin shell assets and added live-mode support to the PWA regression. It retains no license, token, entitlement, or verification URL/response in Cache Storage.
- Made `npm run test:a11y` self-contained: it builds and serves the production site before axe scans it.

## How to run and verify

```sh
npm ci
npm test
npm run build
npm run test:a11y
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm run pack:cli
```

The deployable static docs are in `dist/site/`. The ready-to-publish crate is built with `npm run pack:cli` (which runs `cargo package --allow-dirty`); registry publication remains factory-owned.

To make the review boundary explicit:

```sh
shot-runner plan shots.json
shot-runner run shots.json --allow-command blender --yes
```

Review the displayed `run argv` JSON vector for each shot. With a custom cache path, pass the same `--cache-dir` to both commands.

## Verification completed

| Check | Result |
| --- | --- |
| `npm ci`, `npm test` | PASS — 6 unit and 3 CLI integration tests; site and PWA contracts pass |
| `cargo fmt --all -- --check` and clippy `-D warnings` | PASS |
| `npm run build` | PASS — JS 6.48 KB, CSS 10.43 KB; production docs generated |
| `npm run test:a11y` | PASS — 40 axe passes, 0 violations / 0 serious or critical |
| `npm run pack:cli` | PASS — package verified by Cargo |
| Clean packed consumer | PASS — installed only from `target/package`, planned all 3 argv tokens, rendered with `cp`, and receipt argv exactly matched plan argv; `verify` passed |
| Local PWA regression | PASS — no sensitive Cache Storage entries |
| Live browser check | PASS — 200, title/lang/main/alt checks, no console errors, 390 px axe scan pass |
| Live PWA regression | PASS — active worker and no license/token/entitlement/verify Cache Storage entries |
| Live policy headers | PASS — immutable `assets/*` and content-named proof asset; `sw.js: no-cache`; CSP, frame, Permissions, and COOP headers present |

Lighthouse mobile against the live URL produced 98 performance and 100 accessibility, with FCP 1.16 s, LCP 1.27 s, CLS 0.071, and TBT 94.5 ms. The browser tab crashed while Lighthouse captured its final screenshot after producing the report, so treat those scores as indicative; the independent Playwright/axe and browser checks above completed cleanly.

## Known gaps / next steps

No product blockers are known. A fresh independent release verification can use the deployed URL and the packaged crate. The next content release should change the content-named public asset filename when changing a font or proof image; Vite already hashes generated JS/CSS assets automatically.
