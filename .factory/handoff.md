# Handoff — repair 3

## Outcome

Review 6 findings F-6-1 and F-6-2 are resolved. The product is deployed at <https://animation-shot-runner.sociobot.in/>.

- Implementation commit: `f142407efa699ed789e1955a36140e83de30ffc1`
- Tested and deployed source commit: `875255559bf3a728c10e41ed45d4a115753bf758`
- Static deployment ID: `40820116-baeb-4b92-9ef5-4d421c8f36cb`
- Deployed resource: existing `sf-animation-shot-runner` in `eastus2`

## Repairs

1. The installed CLI help no longer claims that approved renderer commands cannot use a network. It now states only the tested boundary: manifest commands are passed without shell interpretation. A binary-level regression runs `shot-runner --help` and rejects the removed claim.
2. **Reset demo** now exposes its completion message at 390 px. The message is a polite, atomic status update. Its regression focuses the real button, presses Space, observes the isolated storage reset, checks the visible message, and reads the runtime accessibility attributes.
3. The Home and README installation command now pins the repaired implementation commit.

## Cold first read and demo

Fresh 390 × 844 and 1440 × 900 Chromium contexts showed this before scrolling:

- Job: **Render named animation previews from one command.**
- Audience: small animation teams and technical artists who need repeatable local preview renders.
- First action: **Try it with sample data**.

The action opened the Demo in one click. Both viewports showed the persistent sample label, five rendered shots, five cache reuses, receipt verification, and the generated contact sheet. Reset showed its status after Space or Enter. **Start for real** cleared every demo key and preserved a real-storage sentinel.

## Verification

From fresh clone `/tmp/shot-runner-repair3-clean-20260906` at the deployed source commit:

- `npm ci` passed with no reported vulnerabilities.
- Every one of the 18 exact commands in `.factory/claims.json` passed independently.
- `npm test` passed 18 Rust tests, site contracts, all claims, PWA checks, and 390 px browser checks.
- `npm run build` created `target/release/shot-runner` and `dist/site/`.
- `npm run test:a11y` completed 154 route checks with zero axe violations.
- `npm run pack:cli` produced and verified the 18.8 KiB compressed crate.
- `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings` passed.
- A fresh consumer installed the packed crate and exercised help, version, init refusal, five-shot demo, plan, selected cache recovery, and receipt verification.

Live verification covered Home, Demo, Privacy, Terms, and the designed HTTP 404. It found no console errors, broken links, overflow, undersized phone targets, or reduced-motion failures. Live axe had zero violations. Offline reload, route focus/history, same-origin requests, demo cleanup, security headers, cache headers, and local/live file hashes passed.

Mobile Lighthouse: Performance 99, Accessibility 100, Best Practices 100, SEO 100; LCP 1,783 ms, CLS 0, TBT 111 ms. Evidence is under `.factory/evidence/live-repair3/`.

## Earlier findings

- Relative manifest paths, full argv disclosure, trust exits, output recovery, receipt hashes, native contact sheets, and cache reuse remain covered by Rust and claim tests.
- Sensitive service-worker caching, offline reload, immutable asset caching, updateable `sw.js`, and response hardening remain covered locally and live.
- The one-click isolated sample, first-screen copy, route metadata, designed 404, focus/history restoration, terminology, contrast, and 44 px targets remain covered by site, browser, axe, and live checks.
- All 18 retained public claims pass. The untestable CLI no-network claim was removed rather than weakened or relabelled.

## Known limits

Shot Runner runs only executable names the operator approves, but those external renderers may use their own network or license services. Shot Runner does not include or license a renderer. There is no backend, account system, or advertised paid offer in this release.

No known product defect remains from the supplied review history.
