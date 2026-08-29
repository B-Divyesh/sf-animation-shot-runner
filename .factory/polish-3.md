# Polish round 3 — cumulative review resolution

All findings in `review-1.md`, `review-2.md`, and `review-3.md` were rechecked against commit `d22f11db12f2876d39440cd207d601e7310030de` and the deployed site on 2026-08-29. No severity was deferred.

## Round 3 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-3-1 | Replaced the unavailable crates.io command with a pinned Git command, used it in both install locations, and added the visible GitHub install link. | `@claim:install-from-clean-machine` installs in a fresh `CARGO_HOME`, then runs `--version` and `demo`; [live home at 390 px](/work/repo/.factory/evidence/live-home-390.png); live source link returned 200. |
| F-3-2 / F-1-3 | Added `receipt-metadata`, `offline-opened-pages`, `install-from-clean-machine`, `build-output`, and `package-artifact`. Narrowed the account wording, removed the untested MSRV and deployment assertions, and made every registry entry resolve to one tag. | All 17 exact `.factory/claims.json` commands passed independently in a clean clone; `site/tests/site.test.mjs` checks unique IDs and exactly one implementation per tag. |
| F-3-3 | Set the first terminal header to paper text on carbon and fixed the additional dark-section kicker found by the expanded axe run. | `npm run test:browser` computes 16.07:1 for the reported terminal labels; live five-route axe: 155 checks passed, 0 violations; [live demo](/work/repo/.factory/evidence/live-demo-390.png). |
| F-3-4 | Gave wordmark, header/footer links, inline links, and terminal copy controls at least 44 × 44 CSS px. | `npm run test:browser` measures every visible action on five routes at 390 × 844, locally and with `TEST_ORIGIN=https://animation-shot-runner.sociobot.in`. |
| F-3-5 | **Start for real** now removes every `demo:animation-shot-runner:*` key before navigation and preserves non-demo storage. Privacy and demo docs match. | `@claim:isolated-browser-demo` passed locally, from a clean clone, and against the live origin with real and demo sentinels. |
| F-3-6 | Standardised the approval term as **executable name** on the site and in the README. | `site/tests/site.test.mjs` rejects the old term; live page text checked cold. |

## Round 2 findings rechecked

| Finding | Current resolution | Evidence |
| --- | --- | --- |
| F-2-1 / F-1-2 | The first demo viewport contains the real command, five renders, five cache hits, two verified receipt outputs, and the generated contact sheet. | `npm run test:browser` asserts the whole contact sheet stays inside 390 × 844; [live demo](/work/repo/.factory/evidence/live-demo-390.png). |
| F-2-2 / F-1-3 | The assigned test asserts five repeat cache hits, two verified receipt outputs, and tamper failure. | `@claim:demo-cache-and-receipt`. |
| F-2-3 / F-1-3 | The demo uses a new system temporary folder and leaves a caller sentinel unchanged. | `@claim:demo-project-isolation`. |
| F-2-4 / F-1-3 | Planned expanded argv is compared with the receipt’s executed argv. | `@claim:exact-plan-command`. |
| F-2-5 / F-1-3 | A non-demo run must write a copied frame, contact sheet, and receipt. The demo test checks all three for all five shots. | `@claim:run-output-set`; `@claim:demo-five-shot`. |
| F-2-6 / F-1-3 | A separate test proves the second normal run is a cache hit. | `@claim:unchanged-run-cache`. |
| F-2-7 / F-1-3 | Package contents exclude renderer binaries; a native contact sheet succeeds with ffmpeg absent from `PATH`. | `@claim:renderer-dependencies`. |
| F-2-8 / F-1-3 | Copy remains narrowed to observable local inputs and output placement. | `@claim:run-output-set`; `@claim:direct-command-expansion`. |
| F-2-9 / F-1-3 | Hostile literal arguments prove direct argv expansion without shell interpretation. | `@claim:direct-command-expansion`. |
| F-2-10 / F-1-3 | Relative project paths, custom cache paths, and documented exits 0, 2, and 3 are asserted. | `@claim:relative-paths-and-exit-codes`. |
| F-2-11 | Demo, Privacy, Terms, and 404 retain route-specific Twitter title, description, and image metadata. | `site/tests/site.test.mjs`; all live routes returned their distinct titles. |
| F-2-12 | Receipt is defined at first use and every README sentence remains within 22 words. | `.factory/copy-audit.md`; `@claim:receipt-metadata`. |

## Round 1 findings rechecked

| Finding | Current resolution | Evidence |
| --- | --- | --- |
| F-1-1 | The first viewport names the job, audience, sample action, working install, source, and three facts. | `npm run test:browser`; [live phone](/work/repo/.factory/evidence/live-home-390.png); [live desktop](/work/repo/.factory/evidence/live-home-1440.png). |
| F-1-2 | `?demo=1` opens the isolated real demo with banner, Reset, Start for real, bundled sample, and CLI temp run. | `@claim:demo-five-shot`; `@claim:demo-project-isolation`; `@claim:isolated-browser-demo`. |
| F-1-3 | The registry now has 17 individually runnable observable claim tests. | All exact registry commands passed from clean clone `d22f11d`. |
| F-1-4 | Demo, legal, and home routes are real documents; unknown paths return the designed page with HTTP 404. | Live: `/demo/?demo=1` 200, `/privacy/` 200, `/terms/` 200, `/missing-polish-3-cold` 404. |
| F-1-5 | Slogans and ambiguous approval language remain removed; copy is short and terminology is consistent. | `.factory/copy-audit.md`; `site/tests/site.test.mjs`. |
| F-1-6 | Every route retains its own title, description, canonical, OG/Twitter image metadata, touch icon, header, footer, and legal links. | `site/tests/site.test.mjs`; live link crawl returned 200 for every discovered link. |
| F-1-7 | Preview startup, cleanup, service-worker control, and server reachability remain deterministic. | Clean-clone `npm test`; `npm run test:pwa`. |
| F-1-8 | The undefined paid offer remains removed; no unavailable deliverable is sold. | Site and README copy audit; no checkout or license UI in the built output. |

## Round 1 claim sub-findings rechecked

| Finding | Current resolution | Evidence |
| --- | --- | --- |
| F-1-3a | Five named sample shots render. | `@claim:demo-five-shot`. |
| F-1-3b | The demo writes frames, contact sheets, cache entries, and receipts. | `@claim:demo-five-shot`. |
| F-1-3c | Retained local-output statements are observable. | `@claim:run-output-set`. |
| F-1-3d | Commands require review before execution. | `@claim:review-before-run`. |
| F-1-3e | Missing confirmation is rejected. | `@claim:review-before-run`. |
| F-1-3f | Receipt verification detects a changed frame. | `@claim:demo-cache-and-receipt`. |
| F-1-3g | Browser requests and storage isolation are observed. | `@claim:isolated-browser-demo`. |
| F-1-3h | The browser path shows the real CLI workflow, not a validator. | `@claim:demo-five-shot`; live demo screenshot. |
| F-1-3i | The retained no-ffmpeg statement is now tested. | `@claim:renderer-dependencies`. |
| F-1-3j | MIT wording is verified against the distributed license. | `@claim:mit-license`. |
| F-1-3k | The removed billing promise has not returned. | Built-site copy audit. |
| F-1-3l | The removed refund promise has not returned. | Built-site copy audit. |
| F-1-3m | Same-origin browser behavior replaces broad cloud wording. | `@claim:isolated-browser-demo`. |
| F-1-3n | Offline behavior is now both public and registered. | `@claim:offline-opened-pages`; `npm run test:pwa`. |
| F-1-3o | Local output wording is tied to an observed run. | `@claim:run-output-set`. |
| F-1-3p | Direct argv and no-shell behavior are explicit and tested. | `@claim:direct-command-expansion`. |
| F-1-3q | The demo output set is checked for every sample shot. | `@claim:demo-five-shot`. |
| F-1-3r | The demo’s second run reports five cache hits. | `@claim:demo-cache-and-receipt`. |
| F-1-3s | No untestable telemetry or network-code promise is present. | Copy audit; live request log is covered only by the scoped browser claim. |
| F-1-3t | No paid deliverable promise is present. | Built-site copy audit. |
| F-1-3u | Shot-file fields are described and exercised. | `@claim:direct-command-expansion`; `@claim:receipt-metadata`. |
| F-1-3v | Demo contact sheets and receipts are asserted. | `@claim:demo-five-shot`. |
| F-1-3w | No browser manifest-parser claim is present. | Built-site copy audit. |
| F-1-3x | No non-running validator claim is present. | Built-site copy audit; real CLI transcript is live. |
| F-1-3y | No future-download promise is present. | Built-site copy audit. |
| F-1-3z | No paid entitlement promise is present. | Built-site copy audit. |
| F-1-3aa | No merchant-of-record claim is present. | Built-site copy audit. |
| F-1-3ab | No prebuilt-release promise is present. | README audit. |
| F-1-3ac | Renderer separation is explicit and verified. | `@claim:renderer-dependencies`. |
| F-1-3ad | The sample contains five named shots. | `@claim:demo-five-shot`. |
| F-1-3ae | JSON plan output remains parseable. | Clean-clone Rust test `documented_plan_is_parseable`. |
| F-1-3af | Planned argv and recorded argv remain identical. | `@claim:exact-plan-command`. |
| F-1-3ag | Relative manifests resolve from their project directory. | `@claim:relative-paths-and-exit-codes`. |
| F-1-3ah | Public exit codes 0, 2, and 3 are asserted; untested 4/5 wording remains absent. | `@claim:relative-paths-and-exit-codes`. |
| F-1-3ai | Build output is now a registered clean-state claim. | `@claim:build-output`. |
| F-1-3aj | Local read/write behavior is exercised with temporary inputs and outputs. | `@claim:run-output-set`. |
| F-1-3ak | No paid entitlement claim is present. | Built-site copy audit. |

## Final deployed evidence

- Deployment ID: `3f3104b9-bb52-45ef-a936-88f9c929dbf9`.
- `verify-url.sh`: 200, 677 ms, zero console errors, title/lang/one h1/main/alt/button checks passed; report at `.factory/evidence/live-final/verify.json`.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1,222 ms, CLS 0, TBT 0 ms; report at `.factory/evidence/lighthouse.json`.
- Live browser quality: five routes passed 390 × 844 target, overflow, focus, first-screen, and contrast checks.
- Live axe: 155 route checks passed, 0 violations across Home, Demo, Privacy, Terms, and 404.
- Live storage/privacy/offline: `@claim:isolated-browser-demo` and `@claim:offline-opened-pages` passed with `TEST_ORIGIN=https://animation-shot-runner.sociobot.in`.
- Built/live SHA-256 matched for Home, Demo, Privacy, Terms, 404, and `sw.js`.
