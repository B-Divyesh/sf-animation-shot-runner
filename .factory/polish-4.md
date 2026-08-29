# Polish 4 — complete adversarial repair

**Release code commit:** `fd536ab3106e1b314cd4b6735332a2d7d1f526d1`  
**Deployment:** `207d0ad5-7859-4da2-a2e1-e5dbf61c02b7`  
**Live URL:** <https://animation-shot-runner.sociobot.in/>

Every finding from reviews 1–4 and prior polish records was rechecked. No severity was deferred.

## Round 4 findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-4-1 / F-2-12 | Removed the unexplained proof-caption use of “receipt.” The first Home use is now “a JSON receipt that records output hashes, frame rate, and colour space.” The first Demo use defines the same file before its transcript uses the term. Legal/footer copy uses “JSON output records” where it does not need the term. | `site/tests/site.test.mjs`; `@claim:receipt-metadata`; [live Home 390](/work/repo/.factory/evidence/live-polish4-home-390.png); [live Demo 390](/work/repo/.factory/evidence/live-polish4-demo-390.png); live `/` and `/demo/?demo=1` cold checks. |
| F-4-2 | Added history-state scroll/focus snapshots, early pointer activation capture, per-tab route-transition markers, manual scroll restoration, heading focus, and polite announcements. Back and Forward restore the saved scroll/focus state. The privacy page and demo contract disclose the short-lived marker. | `@claim:route-history`; `npm run test:browser`; live `TEST_ORIGIN=https://animation-shot-runner.sociobot.in npm run test:browser`. |
| F-4-3 | Replaced every visible `SR / 01` wordmark with **Shot Runner** and replaced `VOL. 01 — 2026` with **LOCAL CLI · MIT LICENSED**. The phone header drops only the redundant in-page anchor to keep the full wordmark and real demo output in the first viewport. | `site/tests/site.test.mjs`; `npm run test:browser`; live Home/Demo screenshots above. |

## Cumulative finding map

| Finding | Current change or retained resolution | Evidence |
| --- | --- | --- |
| F-1-1 | First screen remains job-led, names small animation teams and technical artists, and makes **Try it with sample data** primary. | `npm run test:browser`; [live Home 390](/work/repo/.factory/evidence/live-polish4-home-390.png). |
| F-1-2 | The one-click `?demo=1` redirect, isolated browser banner/reset/exit controls, real CLI transcript, bundled five-shot contact sheet, and temp-directory CLI demo remain present. | `@claim:demo-five-shot`; `@claim:demo-project-isolation`; `@claim:isolated-browser-demo`; live `/demo/?demo=1`. |
| F-1-3 | The claims registry now contains 18 unique, tagged, clean-state tests, including the new route-history claim. | Every exact command in `.factory/claims.json` passed independently in fresh clone `/tmp/shot-runner-polish4-clean.GFL9Sy/repo`; `site/tests/site.test.mjs`. |
| F-1-4 | Home, Demo, Privacy, and Terms remain real URLs; the configured missing-route response stays a designed HTTP 404. | Live route check: `/` 200, `/demo/?demo=1` 200, `/privacy/` 200, `/terms/` 200, `/missing-polish-4` 404. |
| F-1-5 | Plain-language copy, named actions, defined receipt terminology, and the copy audit remain complete. | `.factory/copy-audit.md`; `site/tests/site.test.mjs`; live cold read. |
| F-1-6 | Route titles, canonical/OG/Twitter metadata, touch icon, shared header/footer, legal links, and build footer remain present. | `site/tests/site.test.mjs`; live route/title check; `/opt/fleet/lib/verify-url.sh`. |
| F-1-7 | Preview server cleanup and PWA/browser suites remain deterministic. | Fresh-clone `npm test`; `npm run test:pwa`; `npm run test:browser`. |
| F-1-8 | No paid offer, checkout link, or unspecified entitlement promise has returned. | Built-site copy audit; fresh-clone `npm test`. |
| F-2-1 | The actual command, output transcript, cache check, receipt verification, and generated contact sheet remain in the first phone Demo viewport. | `npm run test:browser`; [live Demo 390](/work/repo/.factory/evidence/live-polish4-demo-390.png). |
| F-2-2 | Cache hits, receipt verification, and tamper failure remain observed together. | `@claim:demo-cache-and-receipt`. |
| F-2-3 | Caller-folder sentinel and temporary-project isolation remain observed. | `@claim:demo-project-isolation`. |
| F-2-4 | Planned expanded argv still matches the recorded argv. | `@claim:exact-plan-command`. |
| F-2-5 | Normal runs still write copied frames, a contact sheet, and JSON receipt. | `@claim:run-output-set`. |
| F-2-6 | A second unchanged normal run still reuses local cache. | `@claim:unchanged-run-cache`. |
| F-2-7 | Renderer separation and the native no-ffmpeg contact-sheet path remain covered. | `@claim:renderer-dependencies`. |
| F-2-8 | Local-output copy remains scoped to observed behavior. | `@claim:run-output-set`; `@claim:direct-command-expansion`. |
| F-2-9 | Direct argv placeholder expansion remains covered without shell interpretation. | `@claim:direct-command-expansion`. |
| F-2-10 | Relative project paths, custom cache behavior, and documented 0/2/3 exits remain covered. | `@claim:relative-paths-and-exit-codes`. |
| F-2-11 | All five documents retain route-specific Twitter metadata. | `site/tests/site.test.mjs`; live route/title check. |
| F-2-12 | Superseded by the completed F-4-1 terminology repair. | `site/tests/site.test.mjs`; `@claim:receipt-metadata`. |
| F-3-1 | The visible pinned Git install command remains a fresh-machine test path. | `@claim:install-from-clean-machine`. |
| F-3-2 | All retained observable statements are listed; the additional history behavior is now listed too. | Fresh-clone loop of all 18 claim commands. |
| F-3-3 | Demo terminal labels remain at compliant contrast. | `npm run test:browser`; live five-route axe. |
| F-3-4 | Visible phone actions remain at least 44 × 44 CSS px. | `npm run test:browser`; live `TEST_ORIGIN` browser suite. |
| F-3-5 | **Start for real** still clears every demo key while preserving real storage. | `@claim:isolated-browser-demo`; live browser claim. |
| F-3-6 | **Executable name** remains the one approval term. | `site/tests/site.test.mjs`; `.factory/copy-audit.md`. |

## Review 1 claim-subfinding map

| Finding IDs | Change made | Evidence |
| --- | --- | --- |
| F-1-3a, F-1-3b, F-1-3q, F-1-3r, F-1-3v, F-1-3ad | Real five-shot bundled demo, output set, and repeat cache behavior remain shipped. | `@claim:demo-five-shot`; `@claim:demo-cache-and-receipt`. |
| F-1-3c, F-1-3o, F-1-3aj | Public local-output wording remains narrowed to observable local frame/contact-sheet/record output. | `@claim:run-output-set`. |
| F-1-3d, F-1-3e | Plan/review and missing-approval execution boundary remain enforced. | `@claim:review-before-run`. |
| F-1-3f | Changed output frames still fail receipt verification. | `@claim:demo-cache-and-receipt`. |
| F-1-3g, F-1-3m | Browser sample still has no account path, uses demo isolation, and makes same-origin requests only. | `@claim:isolated-browser-demo`; live browser claim. |
| F-1-3h, F-1-3w, F-1-3x | The website remains a transcript/contact sheet of the real CLI demo, not a non-running manifest validator. | `@claim:demo-five-shot`; live Demo screenshot. |
| F-1-3i, F-1-3ac | Renderer separation and native contact-sheet requirements remain explicit and tested. | `@claim:renderer-dependencies`. |
| F-1-3j | MIT wording remains tested against the distributed license. | `@claim:mit-license`. |
| F-1-3k, F-1-3l, F-1-3t, F-1-3y, F-1-3z, F-1-3aa, F-1-3ak | Unavailable billing, refund, entitlement, merchant, and future-delivery promises remain absent. | Built-site copy audit; fresh-clone `npm test`. |
| F-1-3n | Opened docs still reload offline under service-worker control. | `@claim:offline-opened-pages`; live browser claim. |
| F-1-3p | Placeholder expansion remains direct argv, not a shell command. | `@claim:direct-command-expansion`. |
| F-1-3s | The untestable telemetry/network-code promise remains removed; the narrower browser request observation is tested. | Built-site copy audit; `@claim:isolated-browser-demo`. |
| F-1-3u | Shot-file fields and their execution/recording remain described and tested. | `@claim:direct-command-expansion`; `@claim:receipt-metadata`. |
| F-1-3ab | No prebuilt-release-artifact promise has returned. | README and built-site copy audit. |
| F-1-3ae | JSON plan output remains parseable. | Fresh-clone Rust test `documented_plan_is_parseable`. |
| F-1-3af | Planned and recorded argv remain identical. | `@claim:exact-plan-command`. |
| F-1-3ag | Relative manifests resolve from their own project directory. | `@claim:relative-paths-and-exit-codes`. |
| F-1-3ah | Documented success/invalid/missing-approval exit codes remain asserted. | `@claim:relative-paths-and-exit-codes`. |
| F-1-3ai | The documented build outputs remain a registered claim. | `@claim:build-output`. |

## Final evidence

- Fresh remote clone at `fd536ab3106e1b314cd4b6735332a2d7d1f526d1`: `npm ci`, all 18 exact claim commands independently, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `npm test`, `npm run build`, `npm run test:a11y`, and `npm run pack:cli` passed.
- Local mobile Lighthouse: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1,359 ms, CLS 0, TBT 0. Report: `.factory/evidence/lighthouse.json`.
- Live: `/opt/fleet/lib/verify-url.sh` reported 200, zero console errors, correct title/lang/one h1/main/alt/button checks. Live axe reported 154 route checks, 0 violations. Live browser-quality and isolated-demo, route-history, and offline claims passed.
- The deployed Home, Demo, Privacy, Terms, designed 404, and `sw.js` SHA-256 values exactly match `dist/site/`.
