# Polish round 1 — review resolution

Reviewed source: `.factory/review-1.md` and all earlier review, verification, and handoff records present in the repository.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Rewrote the first screen around the actual job, intended users, and a visible sample action. | `site/tests/capture.mjs`; `.factory/evidence/home-390.png` |
| F-1-2 | Added `shot-runner demo`, bundled `examples/paper-courier/`, a real `/demo/?demo=1` route, banner, reset, start-real control, and demo contract. | `@claim:demo-five-shot`; `@claim:isolated-browser-demo`; `.factory/evidence/demo-390.png` |
| F-1-3 | Added `claims.json`, independently runnable tagged claim tests, and removed unprovable paid, telemetry, billing, and future-content copy. | `npm run test:claims`; all commands listed in `claims.json` |
| F-1-4 | Added `/demo/`, root `?demo=1` redirect, designed `404.html`, sitemap entry, and Static Web Apps 404 override. | `site/tests/site.test.mjs`; local route checks in capture suite |
| F-1-5 | Replaced slogan headings, unexplained first-read copy, and ambiguous buttons. Rewrote README in short direct sentences. | `.factory/copy-audit.md`; `.factory/evidence/home-390.png` |
| F-1-6 | Added canonical, Open Graph, Twitter, and touch-icon metadata on every route; made headers/footers consistent. | `site/tests/site.test.mjs` |
| F-1-7 | Rebuilt the PWA test around deterministic readiness, cleanup, offline demo reload, and post-reload server reachability. | `npm run test:pwa` |
| F-1-8 | Removed the undefined Producer Toolkit offer rather than promising unavailable materials. | page/README source audit; `npm run test` |
| F-1-3a | Bundled five named shots render in a fresh temporary folder. | `@claim:demo-five-shot` |
| F-1-3b | The normal demo pipeline writes frames, contact sheets, cache entries, and receipts. | `@claim:demo-five-shot` |
| F-1-3c | Removed broad untested capability ticker; retained demonstrated local outputs. | `@claim:local-files-and-license` |
| F-1-3d | Kept the review-before-run promise with a trust-gate test. | `@claim:review-before-run` |
| F-1-3e | Kept approval copy and covered the missing-confirmation boundary. | `@claim:review-before-run` |
| F-1-3f | Receipt verification catches a tampered demo frame. | `@claim:demo-cache-and-receipt` |
| F-1-3g | Replaced browser-validator promise with browser request/storage isolation coverage. | `@claim:isolated-browser-demo` |
| F-1-3h | Replaced the non-running validator with the real CLI demo. | `@claim:demo-five-shot` |
| F-1-3i | Removed the untested ffmpeg requirement claim from the landing page. | landing copy audit |
| F-1-3j | Retained MIT wording only and tested the distributed license. | `@claim:local-files-and-license` |
| F-1-3k | Removed billing promise. | page/README source audit |
| F-1-3l | Removed refund/revocation promise. | page/README source audit |
| F-1-3m | Replaced broad no-cloud/telemetry copy with observable same-origin sample-page claim. | `@claim:isolated-browser-demo` |
| F-1-3n | Retained offline behaviour as an application test, not marketing copy. | `npm run test:pwa` |
| F-1-3o | Replaced absolute media-leaves-device copy with tested local-output wording. | `@claim:local-files-and-license` |
| F-1-3p | Rewrote shell jargon in README; existing direct command implementation remains covered by Rust tests. | `cargo test --workspace` |
| F-1-3q | Demo output test asserts receipt and contact sheet paths. | `@claim:demo-five-shot` |
| F-1-3r | Demo repeats the same run and asserts five cache hits. | `@claim:demo-five-shot` |
| F-1-3s | Removed untestable no-network-code marketing copy. | README source audit |
| F-1-3t | Removed paid deliverable promise. | page/README source audit |
| F-1-3u | README and UI explain the shot-file fields in plain language. | `cargo test --workspace` |
| F-1-3v | Demo output asserts contact sheet and receipt. | `@claim:demo-five-shot` |
| F-1-3w | Removed browser manifest parser claim. | page source audit |
| F-1-3x | Removed validator claim; the CLI demo now executes the real workflow. | `@claim:demo-five-shot` |
| F-1-3y | Removed future download promise. | page source audit |
| F-1-3z | Removed entitlement claim with the paid offer. | page source audit |
| F-1-3aa | Removed merchant-of-record claim with the paid offer. | page source audit |
| F-1-3ab | Removed prebuilt-release promise. | README source audit |
| F-1-3ac | README says renderer software is separate; terms repeat it. | README/Terms source audit |
| F-1-3ad | Sample manifest contains five named shots. | `@claim:demo-five-shot` |
| F-1-3ae | Existing JSON plan schema integration test remains green. | `cargo test --workspace` |
| F-1-3af | Existing argv parity integration test remains green. | `cargo test --workspace` |
| F-1-3ag | Existing relative-manifest integration tests remain green. | `cargo test --workspace` |
| F-1-3ah | Kept documented exit codes; existing Rust error-path tests remain green. | `cargo test --workspace` |
| F-1-3ai | Replaced build-result prose with commands; clean-clone run recorded in handoff. | `npm ci && npm test && npm run build` |
| F-1-3aj | Retained only local-output claim and tested output placement. | `@claim:local-files-and-license` |
| F-1-3ak | Removed paid entitlement claim. | page/README source audit |

No earlier `polish-*.md` existed. Earlier verification findings for relative manifest paths, safe command disclosure, service-worker sensitive URL handling, headers, cache policy, and axe execution remain covered by the current test suite.
