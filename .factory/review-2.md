# Adversarial first-read review 2 — FAIL

**Reviewed:** 2026-08-28  
**Source candidate:** `3077dc335e70969f61781ee8d53870e1f2571a8b`  
**Live URL:** <https://animation-shot-runner.sociobot.in/>  
**Contexts:** fresh Chromium at 390 × 844 and 1440 × 900; clean clone at `/tmp/shot-runner-review-2.WYtvD6`

## Verdict

**FAIL.** There are blocking demo and claims defects. The CLI itself can run the five-shot sample, but the one-click web path does not immediately show the product being used. The site also makes claim-like statements that have no `claims.json` entry, and one listed claim's assigned test does not test all of that claim. These defects keep the carried-forward F-1-2 and F-1-3 findings open.

## Cold first read

The initial 390 px and desktop screens answer the three first-read questions.

| Question | Cold reading |
| --- | --- |
| What does it do? | It is a local CLI that renders named animation previews from one command. |
| For whom? | Small animation teams and technical artists. |
| What should I click first? | **Try it with sample data**. The nearby note says it opens a five-shot demo. |

The home hero is clear: **“Render named animation previews from one command.”** and **“For small animation teams and technical artists who need repeatable local preview renders.”** The 390 px primary target is 358 × 49 px at y=507; it is visible without scrolling. There was no horizontal overflow or console error in either fresh context.

## Findings

### F-2-1 / F-1-2 — BLOCKING — The one-click sample path does not show the product being used

**Location / evidence:** The home button **“Try it with sample data”** opens `/demo/?demo=1`. On a fresh 390 px visit, the demo banner consumes y=72–245, its h1 y=317–553, and **“See the demo command”** is at y=670. The terminal recording—the only visible result of a run—is at y=2,004–2,223. At 1440 px it starts at y=1,690. The home page has no terminal recording at all.

**Why this fails:** The first screen after the one click shows an explanation and a second navigation-style button, not realistic sample output. It therefore does not meet the CLI demo contract: a landing-page terminal recording of the real command, nor the review requirement that the first demo screen already shows the product in use. The CLI command is real: running it from a separate temporary working directory printed five rendered shots, five cache hits, receipt verification, and a separate `/tmp/shot-runner-demo-*` output folder. That does not repair the web first-read path.

**Concrete fix:** Put the terminal recording, including `shot-runner demo`, five rendered shots, five cache hits, receipt verification, and the temporary output path in the first viewport of `/demo/` on phone and desktop. Also place a compact version or an anchored visible preview beside the landing primary action. Rename **“See the demo command”** to a result verb only if it produces that result, for example **“Copy sample command”**; it must not be required to reach the visible demo result.

### F-2-2 / F-1-3 — BLOCKING — The assigned cache claim test omits half of its claim

**Location / quote:** `.factory/claims.json`, `demo-cache-and-receipt`: **“The bundled demo reuses cached sample frames and its receipt detects a changed frame.”** Its sole command is `npm run test:claims -- --grep @claim:demo-cache-and-receipt`.

**Evidence:** That command passed in the clean clone. It runs only `demo_receipt_detects_a_tampered_sample_frame` in `crates/shot-runner/tests/demo.rs`. The test changes a frame and verifies a hash mismatch, but never reads or asserts `cache_hits_on_repeat`. The cache assertion lives in the test assigned to the different `demo-five-shot` claim.

**Why this fails:** A passing command is not observable proof of the whole registered statement. The registry says each claim has one test tagged for that claim; the cache half is not tested by the assigned tag.

**Concrete fix:** Make the `@claim:demo-cache-and-receipt` invocation run a test that asserts both `cache_hits_on_repeat == 5` and receipt failure after a changed frame, or split this into two claim IDs with one complete test each.

### F-2-3 / F-1-3 — BLOCKING — “Never touches your project” is an unlisted claim

**Location / quote:** Home hero and README: **“It never touches your project.”** / **“Your project files are not used.”**

**Why this fails:** Neither listed claim says the demo avoids a caller's project files. The five-shot test only checks a generated output folder; it does not create sentinel project files in the caller's current directory and prove they remain unchanged.

**Concrete fix:** Add `demo-project-isolation` with a clean temp working directory containing sentinel files. Run `shot-runner demo`, assert the returned directory is under the system temp directory and the sentinels are untouched. List both sentences in its `where` field.

### F-2-4 / F-1-3 — BLOCKING — Exact command disclosure is an unlisted claim

**Location / quote:** Home: **“`plan` shows the exact command.”**

**Why this fails:** `review-before-run` proves missing confirmation blocks execution. It does not assert that `plan` prints the fully expanded command a user will review.

**Concrete fix:** Add a claim and fixture that compares `plan`'s displayed argv with the argv used by `run`, including expanded source, frames, shot, and cache paths.

### F-2-5 / F-1-3 — BLOCKING — Generic output and receipt statements are unlisted

**Location / quote:** Home: **“Each shot gets frames, a contact sheet, and a JSON receipt.”** README: **“Each successful shot writes copied frames, `contact-sheet.png`, and `receipt.json`.”**

**Why this fails:** The five-shot claim covers the bundled sample, not every successful run. `local-files-and-license` relies on source-text matches and does not observe all three outputs.

**Concrete fix:** Add `run-output-set` using a non-demo manifest and assert the three named outputs exist for every completed shot. Point both sentences to it, or narrow the copy to the separately tested bundled demo.

### F-2-6 / F-1-3 — BLOCKING — Generic cache-reuse statements are unlisted

**Location / quote:** Home: **“Repeating unchanged work reuses local cached frames.”** README: **“A second unchanged run uses the local cache.”**

**Why this fails:** The registered cache claim is explicitly about the bundled demo. These statements promise the behaviour for user work, with no listed clean-state test for a normal manifest.

**Concrete fix:** Add `unchanged-run-cache` that runs a non-demo fixture twice and asserts the second result reports the expected cache hits without invoking its renderer; otherwise say this only about the bundled sample.

### F-2-7 / F-1-3 — BLOCKING — Renderer and ffmpeg statements are unlisted claims

**Location / quote:** Home: **“Shot Runner does not include Blender, ffmpeg, or other renderers.”** README: **“Shot Runner does not include a renderer.”**, **“Install and license Blender, Motion Canvas, ffmpeg, or another renderer yourself.”**, and **“Native PNG and JPEG contact sheets do not need ffmpeg.”**

**Why this fails:** None has a registry entry. In particular, the no-ffmpeg statement is a useful dependency promise, but no tagged test proves a native image-only run works with `ffmpeg` unavailable.

**Concrete fix:** Add a package-content/dependency claim for the separate-renderer wording and a PATH-controlled native contact-sheet test for the no-ffmpeg wording. Remove any sentence that cannot be observed from a clean sandbox.

### F-2-8 / F-1-3 — BLOCKING — “Keep media on your computer” is an unlisted privacy claim

**Location / quote:** Home privacy heading: **“Keep media on your computer.”**

**Why this fails:** This is broader than the listed browser-only same-origin request claim and the static local-output test. It reasonably means media is never uploaded during the CLI's whole flow, but no CLI request log or network-denial test is registered.

**Concrete fix:** Add a CLI network-denial test that completes the bundled demo with outbound networking blocked and asserts its media and outputs are only in the returned local directory. Alternatively, narrow the heading to the already proven local-output fact.

### F-2-9 / F-1-3 — BLOCKING — Direct command, shell, and placeholder statements are unlisted claims

**Location / quote:** README: **“The command parts are passed directly to the renderer.”**, **“No shell is used.”**, and **“`{source}`, `{frames}`, `{shot}`, and `{cache}` are replaced before a command runs.”**

**Why this fails:** These are safety and execution promises without a `claims.json` entry. Existing Rust tests may exercise some paths, but they are not assigned claim tests and do not make these visitor-facing statements release-verifiable.

**Concrete fix:** Add a `direct-command-expansion` claim with a hostile-placeholder fixture. Assert the child receives literal argv (no shell interpretation) and exact replacements; list all three README sentences in `where`.

### F-2-10 / F-1-3 — BLOCKING — Relative-path and exit-code statements are unlisted claims

**Location / quote:** README: **“With `shot-runner run project/shots.json`, paths resolve from the `project` folder.”**, **“Use the same `--cache-dir` value with `plan` and `run` when you set one.”**, and **“Exit codes are `0` for success, `2` for a shot-file error, `3` when approval is missing, `4` when the renderer fails, and `5` for output or receipt failures.”**

**Why this fails:** These are exact behavioural contracts, but no registered claim names or tests them. The repository has general integration tests, which is not a substitute for a claims entry for this public copy.

**Concrete fix:** Add `relative-paths-and-exit-codes`, with fixtures for the documented relative run/cache-dir behaviour and each listed failure class, or remove the exact guarantees from README.

### F-2-11 — MEDIUM — Four share routes lack the required Twitter title, description, and image

**Location / evidence:** `site/demo/index.html`, `site/privacy/index.html`, `site/terms/index.html`, and `site/404.html` contain only `<meta name="twitter:card" content="summary_large_image">`. Unlike home, they omit `twitter:title`, `twitter:description`, and `twitter:image`.

**Why this fails:** A shared Demo, legal, or 404 URL does not carry route-specific Twitter presentation even though the site structure contract requires title, description, and real product image for Twitter metadata.

**Concrete fix:** Add the three route-specific Twitter tags to each page, using the existing 1200 × 630 product image. Extend `site/tests/site.test.mjs` to require all four tags on every route.

### F-2-12 — MINOR — README has one overlong, dense sentence and unexplained “receipt” terminology

**Location / quote:** README: **“Exit codes are `0` for success, `2` for a shot-file error, `3` when approval is missing, `4` when the renderer fails, and `5` for output or receipt failures.”** (28 words). Home and README repeatedly say **“JSON receipt”** without stating what the file records.

**Why this fails:** The exit-code sentence exceeds the 22-word hard cap and makes scanning errors harder. “Receipt” is a product-specific word, so a new user cannot tell whether it is a log, report, or provenance file.

**Concrete fix:** Rewrite as: **“Exit 0 means success. Exit 2 means the shot file is invalid. Exit 3 means approval is missing. Exit 4 means the renderer failed. Exit 5 means output or receipt checks failed.”** On first use, write **“JSON receipt, a file that records the output hashes, frame rate, and colour space.”**

## Copy audit

Counts treat command names and hyphenated words as one word. Labels, headings, buttons, status text, and visible prose are included so non-sentence copy can be checked too. `*` identifies copy implicated by a finding above. No landing or README sentence other than R28 exceeds 22 words. No banned marketing adjective was found. The remaining headings name their sections; the primary home button and **Copy command** are result-naming verbs.

### Landing page

| ID | Words | Copy |
| --- | ---: | --- |
| L1 | 7 | Render named animation previews from one command. |
| L2 | 13 | For small animation teams and technical artists who need repeatable local preview renders. |
| L3 | 5 | Try it with sample data |
| L3a | 2 | Copy command |
| L4* | 4 | Opens the five-shot demo. |
| L5* | 5 | It never touches your project. |
| L6 | 4 | Runs on local files |
| L7 | 4 | No account is needed |
| L8 | 2 | MIT-licensed CLI |
| L9 | 6 | Make preview renders you can repeat. |
| L10 | 3 | List your shots |
| L11 | 16 | Put shot names, source paths, frame rate, colour space, and command parts in one JSON file. |
| L12 | 3 | Review the command |
| L13* | 5 | `plan` shows the exact command. |
| L14 | 11 | Nothing runs until you approve the program name and add `--yes`. |
| L15 | 3 | Use the outputs |
| L16* | 11 | Each shot gets frames, a contact sheet, and a JSON receipt. |
| L17* | 7 | Repeating unchanged work reuses local cached frames. |
| L18 | 7 | Run the sample in a temporary folder. |
| L19 | 8 | The demo uses its own bundled scene notes. |
| L20 | 16 | It writes five previews, contact sheets, and receipts, then repeats the run to show the cache. |
| L21 | 4 | Open the sample demo |
| L22 | 6 | Start with your own shot file. |
| L23* | 4 | Install your renderer separately. |
| L24* | 10 | Shot Runner does not include Blender, ffmpeg, or other renderers. |
| L25* | 5 | Keep media on your computer. |
| L26 | 9 | Shot Runner runs local commands and writes local outputs. |
| L27 | 8 | Read the privacy policy for the documentation site. |
| L28 | 5 | Local animation previews with receipts. |
| L29 | 3 | You are offline. |
| L30 | 9 | Opened pages may still be available from this device. |

Non-sentence labels checked: **SR / 01**, **LOCAL PREVIEW RUNNER**, **HOW IT WORKS**, **BUNDLED FIVE-SHOT SAMPLE**, **INSTALL THE CLI**, **PRIVACY**, **FIVE SAMPLE SHOTS / PAPER COURIER**, **24 FPS · sRGB · LOCAL RECEIPTS**, and **BUILT BY PARAM FACTORY / BUILD 2026.08.28**. They are terse product metadata rather than standalone promises; **LOCAL RECEIPTS** should be expanded when the receipt term is first introduced, per F-2-12.

### README

| ID | Words | Copy |
| --- | ---: | --- |
| R1 | 7 | Render named animation previews from one command. |
| R2 | 10 | Shot Runner is for small animation teams and technical artists. |
| R3 | 6 | It runs renderer commands you approve. |
| R4 | 11 | It writes frames, contact sheets, and JSON receipts on your computer. |
| R5 | 10 | Run the five-shot Paper Courier sample after installing from source. |
| R6 | 12 | The command copies bundled sample files into a new system temporary folder. |
| R7 | 5 | It renders five named shots. |
| R8 | 8 | It repeats the run to show cached frames. |
| R9 | 9 | It checks a receipt and prints the output folder. |
| R10* | 6 | Your project files are not used. |
| R11 | 11 | Open `/demo/?demo=1` on the documentation site for the same sample instructions. |
| R12 | 10 | The browser demo marker uses a separate `demo:` local-storage key. |
| R13 | 13 | Use **Reset demo** to reset it or **Start for real** to return home. |
| R14* | 7 | Shot Runner does not include a renderer. |
| R15* | 11 | Install and license Blender, Motion Canvas, ffmpeg, or another renderer yourself. |
| R16* | 10 | Native PNG and JPEG contact sheets do not need ffmpeg. |
| R17 | 16 | A shot file lists a shot name, source path, frame rate, colour space, and command parts. |
| R18* | 9 | The command parts are passed directly to the renderer. |
| R19* | 4 | No shell is used. |
| R20* | 11 | `{source}`, `{frames}`, `{shot}`, and `{cache}` are replaced before a command runs. |
| R21 | 5 | Review the exact command first. |
| R22 | 5 | `plan` does not execute commands. |
| R23 | 8 | `run` requires an approved executable name and `--yes`. |
| R24* | 9 | Each successful shot writes copied frames, `contact-sheet.png`, and `receipt.json`. |
| R25* | 8 | A second unchanged run uses the local cache. |
| R26* | 10 | With `shot-runner run project/shots.json`, paths resolve from the `project` folder. |
| R27* | 13 | Use the same `--cache-dir` value with `plan` and `run` when you set one. |
| R28* | 28 | Exit codes are `0` for success, `2` for a shot-file error, `3` when approval is missing, `4` when the renderer fails, and `5` for output or receipt failures. |
| R29 | 9 | `npm run build` creates the deployable site in `dist/site/`. |
| R30 | 8 | `cargo package --manifest-path crates/shot-runner/Cargo.toml` creates the publishable crate. |
| R31 | 6 | Do not publish from this repository. |
| R32 | 7 | The CLI reads and writes local paths. |
| R33 | 4 | See Privacy and Terms. |
| R34 | 2 | MIT licensed. |
| R35 | 4 | See LICENSE and CHANGELOG.md. |

## Demo, sandbox, claims, structure, and history checks

- **CLI demo:** Passed. From a new temporary working directory, `shot-runner demo` created its own `/tmp/shot-runner-demo-*` folder, rendered five named sample shots, showed five second-run cache hits, verified two receipt outputs, and printed the folder.
- **Browser demo isolation:** Passed. A fresh `/?demo=1` context redirected to `/demo/?demo=1`, showed the required persistent banner, stored only `demo:animation-shot-runner:opened`, reset that key, asked for no account, and made only `https://animation-shot-runner.sociobot.in` requests. No real-browser storage key was present. This does not cure F-2-1's missing first-screen product output.
- **Registered claims:** All five commands in `.factory/claims.json` passed individually in the clean clone: `demo-five-shot`, `demo-cache-and-receipt`, `review-before-run`, `local-files-and-license`, and `isolated-browser-demo`. F-2-2 records the coverage defect despite the pass status.
- **Quality gates:** Clean clone `npm test`, `npm run build`, `npm run test:a11y`, and `npm run pack:cli` passed. Axe reported 39 passes, 0 violations, 0 serious/critical. Built main JavaScript is 0.88 kB gzip and CSS 3.69 kB gzip.
- **Privacy/offline:** The live home and fresh demo request logs were same-origin only. The clean-clone PWA test passed after a service-worker-controlled demo reload while offline. The visible offline message has a clear limitation. No runtime provider key or external CDN was found.
- **Routes and links:** Live `/`, `/demo/?demo=1`, `/privacy/`, `/terms/`, `robots.txt`, `sitemap.xml`, favicon, touch icon, and social image returned 200; a new unknown URL returned the designed `404.html` with HTTP 404. All route titles, descriptions, canonical links, one h1, main landmarks, favicon, touch icon, OG title/description/image, headers, and footers were present. Native document navigation gives usable deep links and browser back. F-2-11 is the remaining Twitter metadata gap.
- **Visual identity and accessibility:** The warm newsprint, instrument-serif masthead, mono metadata, crop-mark red, and proof-contact-sheet art are distinct from a generic SaaS template and match `.factory/design.md`. Mobile targets are at least 44 px, focus is visible, and no 390 px overflow occurred. Motion has a reduced-motion rule in source.
- **Missed leverage / AI:** No finding. The brief is a local repeatable-render CLI; the product already has the obvious bundled sample, JSON receipt, and contact-sheet outputs. An AI feature would not improve the stated core job and should not be added decoratively.
- **Earlier review history:** Read `.factory/review-1.md`, `.factory/polish-1.md`, all three verification reports, and the prior handoff. F-1-1, F-1-4, F-1-6, F-1-7, and F-1-8 were confirmed fixed live and in source. F-1-5's broad first-read copy failure is fixed except F-2-12. F-1-2 and F-1-3 are not fully fixed and are carried forward as F-2-1 through F-2-10.

## What would make this perfect

1. A single tap exposes real five-shot terminal output and the resulting contact sheet in the phone's first demo viewport.
2. Every sentence a visitor can rely on has one complete, clean-state claim test, and every claim test proves its whole wording.
3. Every route has complete Twitter presentation metadata.
4. The README explains its receipt file on first use and keeps every sentence under the 22-word cap.
