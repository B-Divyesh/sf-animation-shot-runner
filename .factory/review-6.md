# Render named animation previews — independent review 6

**Reviewed:** 2026-09-06  
**Verdict:** **FAIL**  
**Finding count:** 2  
**Untested claim count:** 1  
**Implementation candidate:** `29f2312feb8ae2e66637149b9b569dbf9d4b9c86`  
**Documentation reviewed:** `4b5fa6d2ceb1c3612b3b4474caeaf7d97956dfe2`  
**Live URL:** <https://animation-shot-runner.sociobot.in/>

## Verdict

**FAIL.** The live product and packaged CLI complete the main job, and all 18 declared claim commands pass. However, the installed CLI makes one additional public claim that is absent from `.factory/claims.json`. The phone demo also hides all feedback after **Reset demo**, and the changed message is not announced to screen readers at any viewport. A PASS requires zero findings and zero untested claims.

## First screen before scrolling

Fresh Chromium contexts at 390 × 844 and 1440 × 900 showed:

> **Render named animation previews from one command.**  
> For small animation teams and technical artists who need repeatable local preview renders.  
> **Try it with sample data**

- **Job:** Run named local animation preview renders from one command.
- **Audience:** Small animation teams and technical artists.
- **First action:** **Try it with sample data**.

The primary action, its result, the pinned install command, source link, and three plain facts fit in the first phone viewport. Neither viewport had horizontal overflow or a console error.

## Findings

### F-6-1 — BLOCKING — The installed CLI makes an unregistered no-network claim

The packed and installed artifact prints this in `shot-runner --help`:

> No shell and no network are used.

The no-shell behavior is listed and tested by `direct-command-expansion`. No entry in `.factory/claims.json` lists or tests the no-network half. The current suite checks browser requests, but that is a different process and claim. A source audit found no networking crate or `std::net` use, yet an ad hoc source audit is not the required clean-state claim test. The wording is also too broad because Shot Runner launches user-approved renderer commands, and those commands may use a network.

**Required change:** Remove “and no network are used” from public CLI help, or narrow it to an accurate observable statement and add exactly one tagged clean-state claim test. Until then, the untested claim count is 1.

### F-6-2 — MEDIUM — Reset completion is hidden on phone and not announced

At 390 px, keyboard activation with Space successfully recreated `demo:animation-shot-runner:opened`. The resulting message changed to **“Sample view reset. Run the command again to create a new temporary folder.”**, but CSS sets `.demo-banner span { display: none; }`, so the message is invisible. At 1440 px it is visible. At both sizes, `#demo-notice` has no `role="status"` or `aria-live`, so a screen reader is not told that reset completed.

The existing `isolated-browser-demo` claim uses the default desktop viewport and waits for the visible message. It does not cover this phone or screen-reader path.

**Required change:** Keep reset feedback visible at 390 px, make it a polite status message, and add a phone keyboard assertion for visible and announced feedback.

## Demo and real-data isolation

- `/?demo=1` opened `/demo/?demo=1` in one navigation.
- The first demo screen showed the real command, five rendered shots, five cache reuses, receipt verification, and the generated contact sheet.
- The persistent sample label, **Reset demo**, and **Start for real** were present.
- The browser created only `demo:animation-shot-runner:opened`. **Start for real** removed every demo-prefixed key and preserved a real-storage sentinel.
- The full browser flow made only same-origin requests.
- The installed `shot-runner demo` used a new `/tmp/shot-runner-demo-*` folder and left a caller sentinel unchanged.
- The reset operation itself works. F-6-2 concerns its missing user feedback.

## Declared claims

Every exact command in `.factory/claims.json` ran independently after `npm ci` in fresh checkout `/tmp/shot-runner-review6-clean.syumrM/repo` at documentation SHA `4b5fa6d`.

| Claim | Result |
| --- | --- |
| `demo-five-shot` | PASS |
| `demo-cache-and-receipt` | PASS |
| `demo-project-isolation` | PASS |
| `review-before-run` | PASS |
| `exact-plan-command` | PASS |
| `run-output-set` | PASS |
| `unchanged-run-cache` | PASS |
| `receipt-metadata` | PASS |
| `renderer-dependencies` | PASS |
| `direct-command-expansion` | PASS |
| `relative-paths-and-exit-codes` | PASS |
| `isolated-browser-demo` | PASS |
| `route-history` | PASS |
| `offline-opened-pages` | PASS |
| `install-from-clean-machine` | PASS |
| `build-output` | PASS |
| `package-artifact` | PASS |
| `mit-license` | PASS |

F-6-1 is a public CLI-help claim outside that complete registry.

## Installed CLI exercise

The checked `.crate` was installed with `cargo install --path` into fresh consumer directory `/tmp/shot-runner-review6-consumer.Pd6HIJ`.

- `--version` returned `shot-runner 0.1.0`; `--help` documented `init`, `plan`, `run`, `verify`, `demo`, global `--json`, and the exit-safe workflow.
- `init shots.json` succeeded. Repeating it refused overwrite with exit 2.
- `demo --json` rendered five named Paper Courier shots in a new temporary folder and reported five cache hits on repeat.
- `plan --json` returned five shots and complete expanded argv arrays.
- A run without `--yes` returned exit 3. An unknown named shot returned exit 2.
- A selected, approved recovery run returned one cache hit. Its receipt verified two output files.
- The caller directory contained only the deliberately created starter file and unchanged sentinel after the demo.

This covers normal, invalid, boundary, and recovery paths without changing real project data.

## Clean-checkout quality commands

These commands passed from the fresh checkout:

```text
npm ci
npm test
npm run build
npm run test:a11y
npm run pack:cli
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

`npm test` passed 17 Rust tests, all 18 claims, the PWA regression, site contracts, and phone browser checks. `npm run build` produced `target/release/shot-runner` and `dist/site/`. Packaging produced and verified `animation-shot-runner-0.1.0.crate`.

## Live site checks

- Home, Demo, Privacy, and Terms returned 200 with distinct correct titles. An unknown URL returned the designed **Page not found — Shot Runner** document with HTTP 404 and a home action.
- All discovered internal links and assets returned 200. The GitHub source/install link returned 200.
- Each checked route had `lang="en"`, one h1, one main landmark, complete metadata, labelled controls, and image alt text.
- Live axe reported 154 passed route checks and zero violations across five routes.
- Keyboard navigation reached the skip link first with a visible focus ring. Enter opened the sample and **Start for real**; Space activated **Reset demo**. Heading focus, route announcement, and Back/Forward scroll and focus restoration passed.
- Reduced-motion contexts reported no non-zero transition or animation durations.
- The opened Demo route and contact sheet reloaded offline under service-worker control. The service worker remained updateable with `Cache-Control: no-cache`.
- Live HTML, CSS, JavaScript, service worker, and all route documents matched the local candidate build by SHA-256.
- Hashed CSS, JavaScript, image, and font responses used one-year immutable caching. Live responses included CSP, frame denial, COOP, Permissions Policy, referrer policy, HSTS, and content-type protection.
- Mobile Lighthouse scored Performance 99, Accessibility 100, Best Practices 100, and SEO 100. LCP was 1,238 ms, CLS 0, and TBT 77 ms.
- The production build shipped 4.62 KB JavaScript and 14.76 KB CSS before compression. The mobile hero was 29.6 KB and the two fonts totaled 35.7 KB.

There is no product backend, tenant state, account system, or live API allowance. Tenant isolation, restart persistence, health, and 429/Retry-After checks do not apply to this static documentation site and local CLI.

## Earlier findings checked again

| Earlier finding | Current disposition |
| --- | --- |
| Verification 1 P0; Verification 2 P1 | FIXED. Relative manifests execute from their project directory, and `plan` shows the exact argv later recorded by `run`. Clean tests and the installed consumer passed. |
| Verification 1 P1 | FIXED. The current worker accepts only public same-origin files and rejects sensitive query keys and verification paths. No paid or license flow remains. |
| Verification 1 P2 cache; Verification 2 P2 | FIXED. Live hashed assets return one-year immutable caching; `sw.js` returns `no-cache`. |
| Verification 1 P2 accessibility command | FIXED. `npm run test:a11y` creates its evidence directory and passed from the clean checkout. |
| Verification 1 P3; Verification 2 P3 | FIXED. The live CSP, frame denial, COOP, Permissions Policy, referrer policy, HSTS, and content-type headers are present. |
| F-1-1, F-1-5, F-4-3 | FIXED. The current first screen names the job, audience, and first action with a full product wordmark and plain section names. |
| F-1-2, F-2-1, F-2-3, F-3-5 | FIXED at their stated scope. The one-click real sample, temporary CLI folder, namespace separation, and full demo-key removal pass. F-6-2 is a new reset-feedback defect, not a storage-isolation regression. |
| F-1-3 and F-1-3a–F-1-3ak; F-2-2–F-2-10; F-3-2 | The 18 retained registered claims pass, including every original claim sub-finding. F-6-1 is a newly identified unregistered claim in installed CLI help. |
| F-1-4, F-4-2 | FIXED. Real routes, the designed HTTP 404, route announcement, heading focus, and Back/Forward restoration pass live. |
| F-1-6, F-2-11 | FIXED. Route titles, canonical and social metadata, touch icon, shared header/footer, and legal links remain complete. |
| F-1-7 | FIXED. The full clean suite, PWA test, repeated builds, browser suite, and packaging completed without the earlier race. |
| F-1-8 | FIXED. No paid offer, checkout, entitlement, refund, or future-delivery promise is present. |
| F-2-12, F-4-1 | FIXED. Receipt is defined before its first meaningful use on Home, Demo, and README. |
| F-3-1 | FIXED. The visible pinned Git install command passed with a fresh Cargo home and ran the five-shot sample. |
| F-3-3 | FIXED. Demo terminal contrast passes the browser check and axe audit. |
| F-3-4 | FIXED. Visible phone actions meet the 44 × 44 CSS pixel minimum. |
| F-3-6 | FIXED. **Executable name** remains the single approval term. |

No AI step is implied by this local provenance workflow. The useful import, output, repeat, and verification paths already exist; no missed AI feature finding applies.

## Required next step

Resolve F-6-1 and F-6-2, then rerun the exact claims, clean quality chain, packed-consumer flow, and the phone Reset demo check. Do not declare PASS until the public claim registry is complete and reset feedback is visible and announced.
