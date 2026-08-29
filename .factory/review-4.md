# Adversarial first-read review 4 — FAIL

- **Reviewed:** 2026-08-29
- **Candidate:** `ad7a95c348c00e0ccdbcd480927c16f7e3008952`
- **Live URL:** <https://animation-shot-runner.sociobot.in/>
**Contexts:** fresh Chromium at 390 × 844 and 1440 × 900; clean clone at `/tmp/shot-runner-review4-clean.ekot2m/repo`

## Verdict

**FAIL.** One carried copy defect is blocking under the history rule, and two additional findings remain. The first screen and demo are otherwise clear and functional, and all 17 declared claim tests pass independently from a clean clone.

## Cold first read

Before scrolling, both fresh contexts showed **“Render named animation previews from one command.”**, the named audience, **“Try it with sample data”**, its five-shot result, the install command, source link, and three facts.

| Question | Cold answer |
| --- | --- |
| What does this do? | It is a local CLI that renders named animation previews from one command. |
| For whom? | Small animation teams and technical artists who need repeatable local preview renders. |
| What should I click first? | **Try it with sample data**. The nearby text says it opens a five-shot demo and does not touch the visitor's project. |

This gate passes. On the 390 px screen, the primary action ended at y=501 and all three facts ended at y=782. There was no horizontal overflow or console error. The desktop screen gave the same answers.

## Findings

### F-4-1 / F-2-12 — BLOCKING — “Receipt” is still unexplained at its first live uses

**Exact quote / location:** Home proof caption, before the explanatory How it works section: **“24 FPS · sRGB · LOCAL RECEIPTS.”** Demo first screen: **“The real CLI creates a temporary Paper Courier folder and checks its receipt.”** The demo transcript then says **“RECEIPT VERIFIED — 2 output files checked.”**

**Why this fails:** “Receipt” can mean a purchase record. Here it means a JSON record containing output hashes, frame rate, and colour space, but the landing page does not explain that until after its first visible use, and the demo first screen never explains it. Review 2 required the term to be defined on first use. The README now does that, but the live landing and demo paths only half-fix the earlier finding. The review contract requires a half-fixed earlier finding to return as blocking.

**Concrete fix:** Define it at every route's first use. For example, replace the home caption with **“24 FPS · sRGB · JSON RECEIPTS LIST OUTPUT HASHES”** and the demo sentence with **“The real CLI creates a temporary Paper Courier folder and verifies a JSON file of output hashes.”** Add a copy test that finds the first visible occurrence of `receipt` on Home and Demo and confirms that occurrence explains what the file records.

### F-4-2 — HIGH — Route navigation loses scroll and focus state

**Exact location / evidence:** Live Home → Demo → browser Back at 390 × 844. Home was scrolled to y=1200 before the Demo header link was opened. Demo loaded at y=0 with `document.activeElement === document.body`. Browser Back returned to the correct Home URL but y was 0, not 1200, and focus was again `<body>`. Browser Forward repeated the same body focus. `site/src/main.js` contains no route-focus, history-state, `pageshow`, `popstate`, or route announcement handling.

**Why this fails:** A phone visitor who checks the sample from the middle of the landing page loses their place on Back. A keyboard or screen-reader user receives neither restored focus nor focus on the new page heading. This fails the supplied route contract to restore scroll and focus on back/forward and move focus to the new `<h1>` on route changes.

**Concrete fix:** Persist scroll and focused-element state per history entry before internal document navigation. Restore both on `pageshow` for back/forward visits. On a new internal route, focus a programmatically focusable `<h1>` and announce its text in a polite live region. Add a Playwright regression that starts at Home y=1200, visits Demo, asserts Demo h1 focus/announcement, goes Back, and asserts the original URL, scroll position, and focus target are restored.

### F-4-3 — MINOR — The visible wordmark and edition mark are decorative lore

**Exact quote / location:** Every route header: **“SR / 01.”** Home edition line on desktop: **“VOL. 01 — 2026.”**

**Why this fails:** A cold visitor sees an unexplained abbreviation and invented issue numbering instead of the product name. The volume mark gives no product fact. Both conflict with the plain-words rule against decorative labels and brand lore. The `aria-label="Shot Runner home"` helps assistive technology but does not fix the visible copy.

**Concrete fix:** Change the visible wordmark to **“Shot Runner.”** Delete **“VOL. 01 — 2026”** or replace it with a useful fact such as **“LOCAL CLI · MIT LICENSED.”** Keep the broadsheet typography and rule treatment.

## Copy audit

Counts treat hyphenated terms, file names, and command tokens as one word. No landing or README sentence exceeds 22 words, and no banned marketing adjective appears. The flags are in the non-sentence copy table after the sentence lists.

### Landing-page sentences

| Words | Sentence |
| ---: | --- |
| 7 | Render named animation previews from one command. |
| 13 | For small animation teams and technical artists who need repeatable local preview renders. |
| 4 | Opens the five-shot demo. |
| 5 | It never touches your project. |
| 6 | Make preview renders you can repeat. |
| 16 | Put shot names, source paths, frame rate, colour space, and command parts in one JSON file. |
| 5 | `plan` shows the exact command. |
| 11 | Nothing runs until you approve the executable name and add `--yes`. |
| 11 | Each shot gets frames, a contact sheet, and a JSON receipt. |
| 10 | The receipt records output hashes, frame rate, and colour space. |
| 7 | Repeating unchanged work reuses local cached frames. |
| 7 | Run the sample in a temporary folder. |
| 8 | The demo uses its own bundled scene notes. |
| 16 | It writes five previews, contact sheets, and receipts, then repeats the run to show the cache. |
| 6 | Start with your own shot file. |
| 4 | Install your renderer separately. |
| 10 | Shot Runner does not include Blender, ffmpeg, or other renderers. |
| 4 | Write preview output locally. |
| 9 | Shot Runner writes preview output beside your shot file. |
| 8 | Read the privacy policy for the documentation site. |
| 5 | Local animation previews with receipts. |
| 3 | You are offline. |
| 9 | Opened pages may still be available from this device. |

### README sentences

| Words | Sentence |
| ---: | --- |
| 7 | Render named animation previews from one command. |
| 10 | Shot Runner is for small animation teams and technical artists. |
| 6 | It runs renderer commands you approve. |
| 11 | It writes frames, contact sheets, and JSON receipts on your computer. |
| 10 | A receipt records output hashes, frame rate, and colour space. |
| 10 | Run the five-shot Paper Courier sample after installing from GitHub. |
| 12 | The command copies bundled sample files into a new system temporary folder. |
| 5 | It renders five named shots. |
| 8 | It repeats the run to show cached frames. |
| 9 | It checks a receipt and prints the output folder. |
| 6 | Your project files are not used. |
| 7 | Open `/demo/?demo=1` for the same sample output. |
| 7 | The browser demo uses separate `demo:` storage. |
| 5 | **Reset demo** recreates its marker. |
| 10 | **Start for real** discards every demo key and returns home. |
| 7 | Install the pinned source revision from GitHub. |
| 11 | The first command is the tested install path for this release. |
| 6 | See the source and install notes. |
| 7 | Shot Runner does not include a renderer. |
| 11 | Install and license Blender, Motion Canvas, ffmpeg, or another renderer yourself. |
| 10 | Native PNG and JPEG contact sheets do not need ffmpeg. |
| 16 | A shot file lists a shot name, source path, frame rate, colour space, and command parts. |
| 9 | The command parts are passed directly to the renderer. |
| 4 | No shell is used. |
| 11 | `{source}`, `{frames}`, `{shot}`, and `{cache}` are replaced before a command runs. |
| 5 | Review the exact command first. |
| 5 | `plan` does not execute commands. |
| 8 | `run` requires an approved executable name and `--yes`. |
| 10 | A successful run writes copied frames, `contact-sheet.png`, and `receipt.json`. |
| 8 | A second unchanged run uses the local cache. |
| 10 | With `shot-runner run project/shots.json`, paths resolve from the `project` folder. |
| 13 | Use the same `--cache-dir` value with `plan` and `run` when you set one. |
| 4 | Exit 0 means success. |
| 8 | Exit 2 means the shot file is invalid. |
| 6 | Exit 3 means approval is missing. |
| 6 | See `shot-runner --help` for other errors. |
| 10 | `npm run build` creates the release CLI and deployable site. |
| 6 | The outputs are `target/release/shot-runner` and `dist/site/`. |
| 10 | `npm run pack:cli` creates a checked Rust package in `target/package/`. |
| 8 | Do not publish the crate from this repository. |
| 7 | The CLI reads and writes local paths. |
| 4 | See Privacy and Terms. |
| 2 | MIT licensed. |
| 4 | See LICENSE and CHANGELOG.md. |

### Headings, controls, labels, and terminology

| Copy | Result |
| --- | --- |
| **Render named animation previews from one command** | Pass: job-led headline, 7 words. |
| **Try it with sample data** | Pass: required result-led primary action. |
| **Copy command**, **Open source and install on GitHub**, **Open the sample demo** | Pass: each names its result or destination. |
| **Make preview renders you can repeat**, **Run the sample in a temporary folder**, **Start with your own shot file**, **Write preview output locally** | Pass: each heading names its section. |
| **List your shots**, **Review the command**, **Use the outputs** | Pass: concrete step headings. |
| **SR / 01**, **VOL. 01 — 2026** | Fail: decorative or cryptic; F-4-3. |
| **24 FPS · sRGB · LOCAL RECEIPTS** | Fail: first visible use does not define “receipt”; F-4-1 / F-2-12. |
| `shot file`, `executable name`, `preview`, `local cache`, `receipt`, `sample demo` | Consistent terms. README defines receipt immediately; the live first uses do not. |

Other landing fragments checked: **LOCAL PREVIEW RUNNER** (3 words), **Runs on local files** (4), **The sample asks for no account** (6), **MIT-licensed CLI** (2), **HOW IT WORKS** (3), **BUNDLED FIVE-SHOT SAMPLE** (3), **INSTALL THE CLI** (3), **PRIVACY** (1), and **FIVE SAMPLE SHOTS / PAPER COURIER** (5). Each names a product fact or section. Navigation labels name their destinations. README headings **Shot Runner**, **Try the bundled sample**, **Install**, **Use your own shots**, **Develop and verify**, and **Privacy and license** make sense out of context.

The catalog description is 11 words and 73 characters excluding its newline: **“Run repeatable animation previews for named shots from one local command.”** It passes the catalog rule.

## Demo and sandbox

- Home's **Try it with sample data** opened `/demo/?demo=1` in one click.
- The first 390 × 844 demo screen already showed the real `shot-runner demo` transcript, five rendered shots, five repeat cache hits, two checked receipt outputs, and the actual generated contact-sheet image. The image ended at y=843.
- The persistent banner, **Reset demo**, and **Start for real** were present.
- A real-storage sentinel survived entry, Reset, and exit. Reset recreated only `demo:animation-shot-runner:opened`. Start for real removed every `demo:animation-shot-runner:*` key.
- The browser flow made only `https://animation-shot-runner.sociobot.in` requests. There were no analytics, CDN, model-provider, or key-bearing requests.
- A separate CLI run from `/tmp/shot-runner-review4-caller.*` rendered five shots, reported five repeat cache hits, and wrote five receipts and five contact sheets under `/tmp/shot-runner-demo-*`. The caller directory still contained only `real-project-sentinel`.
- The live offline claim passed through its assigned browser test. The opened demo and contact sheet reloaded after the context was set offline.

The demo passes the sandbox contract. F-4-1 concerns the explanation of its receipt output, not whether the output exists.

## Claims audit

Every exact command was run independently after `npm ci` in clean clone `/tmp/shot-runner-review4-clean.ekot2m/repo` at `ad7a95c348c00e0ccdbcd480927c16f7e3008952`.

| Claim | Result | Observed scope |
| --- | --- | --- |
| `demo-five-shot` | PASS | Five named shots and each output set. |
| `demo-cache-and-receipt` | PASS | Five cache hits, two verified outputs, changed-frame failure. |
| `demo-project-isolation` | PASS | Caller sentinel and temp-directory boundary. |
| `review-before-run` | PASS | Missing approval prevents execution. |
| `exact-plan-command` | PASS | Planned expanded argv equals recorded argv. |
| `run-output-set` | PASS | Frame, contact sheet, and receipt from a normal run. |
| `unchanged-run-cache` | PASS | Second normal run reports one cache hit and no render. |
| `receipt-metadata` | PASS | Source/output hashes, 23.976 fps, Display P3, and verification. |
| `renderer-dependencies` | PASS | No bundled renderer; native contact sheet with empty `PATH`. |
| `direct-command-expansion` | PASS | All placeholders expand as direct argv without shell interpretation. |
| `relative-paths-and-exit-codes` | PASS | Nested project, custom cache, and exits 0, 2, and 3. |
| `isolated-browser-demo` | PASS | No account field, demo namespace, cleanup, real sentinel, same-origin requests. |
| `offline-opened-pages` | PASS | Controlled offline demo reload and image availability. |
| `install-from-clean-machine` | PASS | Exact pinned Git command, version, and five-shot demo in fresh Cargo state. |
| `build-output` | PASS | Release CLI and all deployable routes created from clean output. |
| `package-artifact` | PASS | Non-empty checked `.crate` artifact. |
| `mit-license` | PASS | Distributed MIT grant. |

The landing, README, Demo, Privacy, and Terms claim-like statements were cross-checked against the registry and assigned tests. No unlisted behavioral, privacy, offline, quantitative, or dependency claim remains. F-4-1 is a plain-language definition failure, not missing receipt test coverage.

## Structure, links, accessibility, and identity

- Live Home, Demo, Privacy, and Terms returned 200. `/missing-review-4-probe` returned the designed 404 with HTTP 404.
- Every route had `lang=en`, one h1, one main, ordered headings, route-specific title and description, canonical, SVG favicon, 180 × 180 touch icon, OG/Twitter metadata, and the 1200 × 630 product image.
- The Home title is **“Shot Runner — local animation previews.”** Demo, Privacy, Terms, and 404 use the required route-first title pattern.
- Every discovered internal link and fragment resolved. The GitHub source/install link returned 200. Header/footer navigation and legal links were consistent.
- Browser Back and Forward reached the correct URLs, but scroll and focus restoration failed as recorded in F-4-2.
- Live `verify-url.sh` returned HTTP 200, zero console errors, correct title/lang, one h1, one main, and no missing alt text or unnamed buttons.
- Live axe ran 155 route checks across five routes with zero violations. The 390 px browser check found no undersized targets, overflow, console errors, or low-contrast terminal labels.
- Reduced-motion rules remove transitions, animations, and smooth scrolling. Initial JavaScript is 2,113 bytes raw; CSS is 14,431 bytes raw.
- Live headers include the self-only CSP, framing protections, restrictive permissions policy, referrer policy, and `nosniff`. `sw.js` is `no-cache`; hashed assets are immutable.
- Built/live SHA-256 values matched for Home, Demo, Privacy, Terms, 404, and `sw.js`.
- The newsprint/carbon palette, serif/mono type, hard rules, contact sheet, and crop-mark red form a distinct production-broadsheet identity. It is not a generic SaaS template.

## Earlier finding audit

Every earlier review, polish report, verification report, and handoff was read. Current status was checked against both live behavior and source.

| Earlier ID | Current verification |
| --- | --- |
| F-1-1 | FIXED — cold phone and desktop screens state job, audience, action, result, and facts. |
| F-1-2 | FIXED — one-click live output, browser sandbox, Reset/exit, and temp CLI demo all pass. |
| F-1-3 | FIXED — 17 claims exist, have one tag each, and all exact commands pass independently. |
| F-1-4 | FIXED at its original scope — real Demo/legal routes and HTTP 404 work. F-4-2 is a new scroll/focus state defect. |
| F-1-5 | FIXED at its original slogan/button scope. F-4-3 identifies remaining decorative labels. |
| F-1-6 | FIXED — route metadata and shared header/footer are live. |
| F-1-7 | FIXED — clean `npm test` and PWA checks are deterministic. |
| F-1-8 | FIXED — no undefined paid offer, checkout, or entitlement UI remains. |
| F-2-1 / F-1-2 | FIXED — transcript and contact sheet fit the first demo viewport. |
| F-2-2 / F-1-3 | FIXED — assigned test covers cache hits, receipt verification, and tamper failure. |
| F-2-3 / F-1-3 | FIXED — caller sentinel and temp boundary pass. |
| F-2-4 / F-1-3 | FIXED — planned and recorded expanded argv match. |
| F-2-5 / F-1-3 | FIXED — normal output set has a registered passing test. |
| F-2-6 / F-1-3 | FIXED — normal second-run cache reuse has a registered passing test. |
| F-2-7 / F-1-3 | FIXED — separate renderer and no-ffmpeg native path are tested. |
| F-2-8 / F-1-3 | FIXED — public wording is narrowed to tested local output behavior. |
| F-2-9 / F-1-3 | FIXED — hostile direct-argv fixture proves no shell interpretation. |
| F-2-10 / F-1-3 | FIXED — relative project/cache behavior and advertised exits pass. |
| F-2-11 | FIXED — route-specific Twitter metadata is present on all five documents. |
| F-2-12 | **NOT FIXED — carried as blocking F-4-1 / F-2-12.** README defines receipt, but live Home and Demo use it first without a definition. |
| F-3-1 | FIXED — exact visible pinned install command passes in a fresh Cargo home. |
| F-3-2 / F-1-3 | FIXED — receipt, account, offline, install, build, and package statements are registered and tested. |
| F-3-3 | FIXED — demo terminal labels measure at compliant contrast; live regression passes. |
| F-3-4 | FIXED — all visible phone actions measure at least 44 × 44 CSS px. |
| F-3-5 | FIXED — Start for real clears the full demo namespace and preserves real keys. |
| F-3-6 | FIXED — **executable name** is used consistently. |

The Appendix C subfindings from review 1 were each rechecked:

- F-1-3a, F-1-3b, F-1-3q, F-1-3r, F-1-3v, and F-1-3ad: fixed by the two demo tests and live sample evidence.
- F-1-3c, F-1-3o, and F-1-3aj: fixed by scoped local-output wording and `run-output-set`.
- F-1-3d and F-1-3e: fixed by `review-before-run`.
- F-1-3f: fixed by `demo-cache-and-receipt` tamper detection.
- F-1-3g and F-1-3m: fixed by the whole-flow same-origin request and storage test.
- F-1-3h, F-1-3w, and F-1-3x: fixed by replacing the validator with the real CLI sample output.
- F-1-3i and F-1-3ac: fixed by `renderer-dependencies`.
- F-1-3j: fixed by `mit-license` and removal of paid-core wording.
- F-1-3k, F-1-3l, F-1-3t, F-1-3y, F-1-3z, F-1-3aa, and F-1-3ak: fixed by removal of the paid offer and its promises.
- F-1-3n: fixed by `offline-opened-pages`.
- F-1-3p: fixed by `direct-command-expansion`.
- F-1-3s: fixed by removal of the broad network-code claim; the narrower browser request claim passes.
- F-1-3u: fixed by shot-file field copy and direct/receipt fixtures.
- F-1-3ab: fixed by removal of the prebuilt-binary promise.
- F-1-3ae: fixed by the parseable JSON-plan test in the clean suite.
- F-1-3af: fixed by `exact-plan-command`.
- F-1-3ag and F-1-3ah: fixed by `relative-paths-and-exit-codes`.
- F-1-3ai: fixed by `build-output` and `package-artifact`.

The earlier verification defects for relative manifests, full argv disclosure, live cache headers, security headers, service-worker cache exclusions, and clean accessibility execution remain fixed.

## Quality gates

The clean clone passed:

- `npm test`: 8 library tests, 3 demo tests, 6 integration tests, site contracts, all 17 claims, PWA/offline, and browser-quality checks.
- `npm run build`: release CLI plus `dist/site/`.
- `npm run test:a11y`: 155 checks, zero axe violations across five routes.
- `npm run pack:cli`: 15 files, 74.8 KiB unpacked, 18.7 KiB compressed; Cargo verification passed.

These passes do not cover F-4-1, F-4-2, or F-4-3. In particular, the browser suite checks target size and first-screen fit but not route focus/scroll restoration or first-use terminology.

## Missed leverage

No finding. The brief calls for a local manifest runner with cached frames, contact sheets, and receipts; those capabilities, a bundled sample, and a JSON artifact already exist. AI would be decorative for this deterministic workflow. Remote sync would conflict with the local-first boundary, and the JSON shot file already supplies the import/export format.

## What would make this perfect

1. Define **receipt** at its first visible use on both Home and Demo, with a regression for first-use clarity.
2. Preserve scroll and focus on Back/Forward, focus and announce the new h1 on forward route navigation, and test the sequence at 390 px.
3. Replace **SR / 01** with **Shot Runner** and remove or factualize **VOL. 01 — 2026**.

Until all three are complete and the full checklist is rerun, the zero-findings standard is not met.
