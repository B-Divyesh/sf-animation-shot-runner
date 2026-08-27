# Shot Runner v0.1.0 — handoff

## What shipped

- Rust single-binary CLI (`shot-runner`) with `init`, inert `plan`, explicitly trusted `run`, selected-shot runs, `--json`, and receipt `verify` commands.
- JSON manifests for named shots with source, FPS, colorspace, and tokenized renderer arguments. Commands never pass through a shell. Execution requires both `--yes` and an exact repeated `--allow-command` match.
- SHA-256 source hashing for files or directories, content-addressed local frame caches, deterministic native PNG/JPEG contact sheets, copied preview frames, and JSON receipts with source/manifest/output hashes.
- A responsive static documentation site in `dist/site/`, including a local-only five-shot manifest planner, explicit empty/loading/error/offline states, keyboard/focus treatment, install documentation, privacy/terms routes, and an offline service worker.
- Optional $39 Producer Toolkit checkout through Sociobot only. The site stores returned tokens under `sb_license:animation-shot-runner`, strips the query token, verifies at most daily, restores pasted licenses, retains cached access offline, and never gates the complete CLI, safety, accessibility, or receipt export.
- An original generated contact-sheet hero in responsive 146 KB and 29 KB WebP variants, plus two self-hosted Latin WOFF2 fonts totaling 35.7 KB. Provenance and the exact image prompt are in `.factory/design.md`.

## Run and verify

```sh
npm install
npm test
npm run build
npm run pack:cli
```

The exact build command is `npm run build`; the static deploy root is `dist/site/`, with `dist/site/index.html` at its root. The release binary is `target/release/shot-runner`. `npm run pack:cli` verifies and writes the ready-to-publish crate under `target/package/`; registry publication is intentionally left to the factory.

Additional checks run on 2026-08-27:

- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `npm test`: pass, including five Rust tests. The end-to-end test executes a real allowlisted local command, creates a frame/contact sheet/receipt, verifies all hashes, then proves the second run is a cache hit.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4173/ .factory/evidence`: HTTP 200, one h1, title/lang/main/alt checks pass, and zero browser console errors at desktop and 390 px.
- `npm run test:a11y`: axe Playwright scan, 40 rules passed, zero violations and zero serious/critical findings.
- Lighthouse mobile: Performance 98, Accessibility 100, Best Practices 100, SEO 100; FCP 1.0 s, LCP 2.3 s, CLS 0, total blocking time 0 ms. INP is not produced for a lab navigation; total blocking time is the lab responsiveness proxy.
- Production asset budgets: initial JS 6.3 KB, CSS 10.3 KB, loaded WOFF2 fonts 35.7 KB, mobile hero 29 KB, desktop hero 146 KB.

## Known gaps and release notes

- The factory still needs to register the Sociobot product and publish release binaries/crate artifacts. No product ID or payment-provider integration is embedded here.
- Real Blender/Motion Canvas/ffmpeg renders were not run in this container. Their installation and licensing remain the operator’s responsibility and are surfaced in both README and site copy. The renderer-agnostic execution boundary is covered end to end with a local command fixture.
- Bit-for-bit output reproduction ultimately depends on the selected renderer, its version, and deterministic project inputs. Shot Runner records hashes and detects drift but cannot make a nondeterministic third-party renderer deterministic.
