# Render named animation previews — independent verification 4

**Verified:** 2026-09-06  
**Verdict:** **FAIL**  
**Finding count:** 1  
**Untested claim count:** 3  
**Implementation candidate:** `f142407efa699ed789e1955a36140e83de30ffc1`  
**Deployed source:** `875255559bf3a728c10e41ed45d4a115753bf758`  
**Documentation reviewed:** `3f031a14faf5a9ec160f75ef2e6f7ab97c212abf`  
**Deployment:** `40820116-baeb-4b92-9ef5-4d421c8f36cb`  
**Live URL:** <https://animation-shot-runner.sociobot.in/>

## Verdict

**FAIL.** The live site, one-click sample, packed CLI, and both review-6 repairs work. Every one of the 18 declared claim commands passes. However, the installed CLI and README retain three public statements without complete entries and observable tagged tests in `.factory/claims.json`. The supplied contract requires zero untested public claims for PASS.

No product code was changed during this verification.

## First screen before scrolling

Fresh Chromium contexts at 390 × 844 and 1440 × 900 showed:

> **Render named animation previews from one command.**  
> For small animation teams and technical artists who need repeatable local preview renders.  
> **Try it with sample data**

- **Job:** Render named animation previews from one local command.
- **Audience:** Small animation teams and technical artists.
- **First action:** **Try it with sample data**. Its adjacent text says it opens the five-shot demo and does not touch the visitor's project.

The action, pinned install command, source link, and three plain facts fit in the first phone viewport. Both viewports had no horizontal overflow or console errors.

## Finding

### V4-1 — BLOCKING — Three public CLI claims lack complete registered tests

The installed artifact prints these statements:

1. `init` — **“Write a documented starter manifest (never overwrites)”**.
2. Main help — **“caches by source content”**.
3. `plan` — **“without executing”**. The README also says **“plan does not execute commands.”**

None has a complete claim entry with exactly one matching tagged test:

- No claim entry covers starter-manifest creation or overwrite refusal. The current suite has no committed regression for the refusal path.
- `unchanged-run-cache` proves only that identical input reuses a cache. It does not change source contents and prove a cache miss, so it does not establish **“caches by source content.”**
- `review-before-run` calls `run` without approval. `exact-plan-command` compares planned and recorded argv, but it does not assert that `plan` produced no renderer or output side effect. The public no-execution statement is therefore only partially covered.

The packed binary behaved correctly in this verification: a second `init` exited 2 without overwriting, a source change changed a cache hit into a render, and `plan` did not visibly write preview output. Ad hoc verification does not replace the required registry entries and clean-state tagged regressions.

**Required change:** Add one accurately scoped claim and one tagged observable test for each statement, or remove/narrow the public wording. The `plan` test must use a side-effecting fixture and assert the side effect and output directory are absent after `plan`.

## Declared claims

A fresh remote clone was checked out at documentation SHA `3f031a1`. `npm ci` installed 20 packages with zero reported vulnerabilities. Every exact command in `.factory/claims.json` then ran independently.

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

Declared claim failures: **0**. Untested public claims found by cross-check: **3**.

## Live sample and real-data isolation

- `/?demo=1` opened `/demo/?demo=1` in one keyboard action.
- The first phone screen showed the real command, five rendered shots, five cache reuses, two verified receipt outputs, and the generated contact sheet.
- The label **“Demo — sample data, nothing is saved”** remained present with **Reset demo** and **Start for real**.
- At 390 px, Space on **Reset demo** recreated only `demo:animation-shot-runner:opened`. The completion text was visible and exposed as `role="status"`, `aria-live="polite"`, and `aria-atomic="true"`.
- **Start for real** removed every `demo:animation-shot-runner:*` key and preserved a non-demo sentinel.
- All 35 observed requests in the complete phone flow were same-origin.
- The service worker updated successfully, remained active, cached only public same-origin files, and reloaded the opened Demo and contact sheet offline.

This proves F-6-2 is fixed and that the browser sample does not change real stored data.

## Installed CLI exercise

`npm run pack:cli` produced and verified `animation-shot-runner-0.1.0.crate`. It was installed with `cargo install --path ... --root ... --locked` into a fresh consumer directory.

- `--version` returned `shot-runner 0.1.0`.
- `--help` listed `init`, `plan`, `run`, `verify`, `demo`, and `--json`. It says only **“No shell is used to interpret manifest commands.”** The removed no-network statement is absent, proving F-6-1 fixed.
- `demo --json` rendered five named Paper Courier shots under a new `/tmp/shot-runner-demo-*` folder and reported five cache hits on repeat.
- All five shot folders contained a frame, contact sheet, and receipt. The caller folder retained only its sentinel before real setup began.
- A receipt verified two outputs. A changed frame failed with exit 5; a selected cache recovery returned one cache hit and restored successful verification.
- A normal starter file was created. A repeated `init` exited 2, missing `--yes` exited 3, and an unknown shot exited 2.
- `fps: 0.001` planned successfully. `fps: 0` exited 2 with a direct recovery message.
- A first cache test rendered once, an unchanged second run returned one cache hit, and changing source contents caused a new render.

Normal, invalid, boundary, and recovery paths work. V4-1 concerns missing permanent claim registration and regression coverage, not a reproduced runtime failure.

## Quality, accessibility, privacy, and routes

- `npm test` passed 18 Rust tests, site contracts, all declared claims, PWA checks, and 390 px browser checks.
- `npm run build` created `target/release/shot-runner` and `dist/site/`.
- `npm run test:a11y` and the same suite against live each completed 154 axe checks across five routes with zero violations.
- `npm run pack:cli`, `cargo fmt --all -- --check`, and `cargo clippy --workspace --all-targets -- -D warnings` passed.
- The factory URL verifier returned HTTP 200, no console errors, `lang=en`, one h1, one main, complete image alternatives, and labelled buttons.
- Keyboard traversal reached the skip link, navigation, sample action, install-copy action, source link, and sample link with 3 px focus outlines. Enter and Space operated their controls.
- At 200% root text size, the 390 px page had no horizontal loss and kept the primary action visible.
- Reduced motion computed zero-second animation and transition durations.
- Home, Demo, Privacy, and Terms returned 200 with distinct correct titles. A missing route deliberately returned the designed **Page not found — Shot Runner** page with HTTP 404 and a home action.
- All discovered internal pages/assets and the GitHub install link returned 200. Fragment targets exist.
- Live headers include CSP, frame denial, COOP, Permissions Policy, referrer policy, HSTS, and content-type protection. `sw.js` is `no-cache`; hashed CSS and JavaScript use one-year immutable caching.
- The CLI has no backend, account, tenant, or server state. Backend tenant, restart, health, and 429 checks do not apply.

The live Home, Demo, Privacy, Terms, 404, service worker, hashed CSS/JavaScript, and self-hosted font bytes matched the clean local build. This establishes that the static deployment is the output of deployed source `8752555`; the product implementation remains `f142407`, while `3f031a1` changes only final documentation.

## Performance

Fresh live mobile Lighthouse results:

- Performance: **100**
- Accessibility: **100**
- Best Practices: **100**
- SEO: **100**
- LCP: **1,222 ms**
- CLS: **0**
- TBT: **0 ms**

The production build contains 4,655 bytes of JavaScript, 14,789 bytes of CSS, 35,740 bytes of fonts, and a 29,572-byte mobile proof image.

## Earlier finding disposition

| Earlier finding | Current evidence |
| --- | --- |
| Verification 1 P0; Verification 2 P1 | FIXED — relative manifests run from their project folder; complete planned argv matches the receipt. |
| Verification 1 P1 | FIXED — the worker caches only public same-origin files; no paid/license flow remains. |
| Verification 1 P2 cache; Verification 2 P2 | FIXED — live hashed assets are immutable and `sw.js` is updateable. |
| Verification 1 P2 accessibility command | FIXED — the clean accessibility command created evidence and passed. |
| Verification 1 P3; Verification 2 P3 | FIXED — live hardening headers are present. |
| F-1-1, F-1-5, F-4-3 | FIXED — cold phone and desktop screens state the job, audience, first action, useful facts, and full wordmark in plain words. |
| F-1-2, F-2-1, F-2-3, F-3-5 | FIXED — one-click real sample, populated first screen, temporary CLI folder, separate browser namespace, reset, and exit cleanup pass. |
| F-1-3 and F-1-3a–F-1-3ak; F-2-2–F-2-10; F-3-2 | The 18 retained entries pass. **NOT COMPLETE at the overall claim-contract level:** V4-1 identifies three additional public CLI statements without complete registered tests. F-1-3d's no-execution statement is specifically incomplete. |
| F-1-4, F-4-2 | FIXED — real routes, designed HTTP 404, heading announcement, and Back/Forward scroll/focus restoration pass live. |
| F-1-6, F-2-11 | FIXED — titles, metadata, header/footer, legal pages, icons, and links remain complete. |
| F-1-7 | FIXED — clean full test, build, browser, PWA, and package runs are deterministic. |
| F-1-8 | FIXED — no paid offer, checkout, entitlement, refund, or future-delivery promise remains. |
| F-2-12, F-4-1 | FIXED — receipt is defined before its first meaningful use. |
| F-3-1 | FIXED — the visible pinned Git command installs and runs the five-shot demo in fresh Cargo state. |
| F-3-3, F-3-4, F-3-6 | FIXED — contrast, 44 px phone targets, and consistent **executable name** terminology pass. |
| F-6-1 | FIXED — installed help contains no no-network claim; the binary regression also passed. |
| F-6-2 | FIXED — phone reset feedback is visible, keyboard-operated, and announced as an atomic polite status. |

No AI feature is implied by this local render-provenance job. Adding one would not improve the stated workflow.

## Evidence

- Clean checkout: `/tmp/shot-runner-verify4-clean`
- Clean installed consumer: `/tmp/shot-runner-verify4-consumer`
- Live screenshots, URL verification, reset state, and Lighthouse: `/work/.evidence/verify4/`

## Required next step

Register and test the three statements in V4-1, then rerun every exact claim command and the clean package/consumer flow. Do not declare PASS while any public claim lacks its required tagged test.
