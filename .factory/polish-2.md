# Polish round 2 — review resolution

This round read `.factory/review-1.md`, `.factory/review-2.md`, `.factory/polish-1.md`, and the verification records. Every outstanding finding is resolved below.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1 / F-1-2 | Put the real `shot-runner demo` transcript and an actual generated Paper Courier contact sheet in the first demo viewport. The command is directly copyable. | `.factory/evidence/demo-390.png`; `site/tests/capture.mjs`; `/demo/?demo=1` after deploy |
| F-2-2 / F-1-3 | The cache/receipt test now asserts all five repeated-run cache hits and a tampered-frame receipt failure. | `@claim:demo-cache-and-receipt` |
| F-2-3 / F-1-3 | Added a sentinel caller-folder test and the `demo-project-isolation` claim. | `@claim:demo-project-isolation` |
| F-2-4 / F-1-3 | Registered the plan/recorded-argv promise and test. | `@claim:exact-plan-command` |
| F-2-5 / F-1-3 | Registered non-demo frame, contact-sheet, and receipt output coverage. | `@claim:run-output-set` |
| F-2-6 / F-1-3 | Registered non-demo second-run cache coverage. | `@claim:unchanged-run-cache` |
| F-2-7 / F-1-3 | Registered separate-renderer/native contact-sheet wording; package listing confirms no renderer binary. | `@claim:renderer-dependencies` |
| F-2-8 / F-1-3 | Narrowed broad media wording to the observable local preview-output behavior. | `@claim:run-output-set`; home source audit |
| F-2-9 / F-1-3 | Added hostile argv fixture proving placeholder expansion and no shell interpretation. | `@claim:direct-command-expansion` |
| F-2-10 / F-1-3 | Added a nested relative-manifest claim test for documented paths and 0/2/3 exits; removed untested 4/5 exact wording. | `@claim:relative-paths-and-exit-codes` |
| F-2-11 | Added title, description, and image Twitter tags on Demo, Privacy, Terms, and 404; site contract test requires them. | `site/tests/site.test.mjs` |
| F-2-12 | Defined the JSON receipt on first use and split the exit-code text into short sentences. | README and home copy audit |

Local visual checks: [mobile demo](/work/repo/.factory/evidence/demo-390.png), [mobile home](/work/repo/.factory/evidence/home-390.png), and [desktop home](/work/repo/.factory/evidence/home-1440.png) show the repaired broadsheet design without horizontal overflow or console errors. Cold live confirmation used `https://animation-shot-runner.sociobot.in/demo/?demo=1`, `/`, `/privacy/`, `/terms/`, and `/missing-polish-2` (HTTP 404); [live mobile demo](/work/repo/.factory/evidence/live-demo-390.png) confirms F-2-1 after deployment.
