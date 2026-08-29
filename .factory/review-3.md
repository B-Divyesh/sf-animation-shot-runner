# Adversarial first-read review 3 — FAIL

**Reviewed:** 2026-08-29

**Candidate:** `224935b96570655b27fbf4e26d39dedaaad87cc8`

**Live URL:** <https://animation-shot-runner.sociobot.in/>

**Contexts:** fresh Chromium at 390 × 844 and 1440 × 900; clean clone at `/tmp/shot-runner-review-3.pDIZPy/repo`

## Verdict

**FAIL.** The one-click sample is now clear and useful, but the real install command shown in the first screen does not work. The deployed demo also has a 1.10:1 text-contrast defect, undersized phone targets, and demo state that survives **Start for real**. Several public statements still have no complete entry in `.factory/claims.json`, so the earlier claims finding is not fully fixed.

## Cold first read

Before scrolling, both fresh contexts answered the three required questions:

| Question | Cold reading |
| --- | --- |
| What does it do? | It renders named local animation previews from one command and produces local output. |
| For whom? | Small animation teams and technical artists. |
| What should I click first? | **Try it with sample data**; the adjacent text says it opens a five-shot demo without touching the project. |

The decisive text is **“Render named animation previews from one command.”**, **“For small animation teams and technical artists who need repeatable local preview renders.”**, and **“Try it with sample data.”** The primary action was visible at y=507 on the 390 px page and y=702 on desktop. This gate passes.

The first demo viewport also passes its presentation gate. It contains the persistent demo banner, `shot-runner demo`, five rendered shots, five cache hits, receipt verification, and the generated contact sheet without a second click.

## Findings

### F-3-1 — BLOCKING — The advertised install command cannot install the product

**Location / quote:** first screen, beside the primary demo action: `cargo install animation-shot-runner`.

**Evidence:** From a new temporary directory, the exact displayed command with only a temporary install root added, `cargo install animation-shot-runner --root /tmp/shot-runner-review-3.pDIZPy/crates-install-exact`, exited 101 with:

```text
error: could not find `animation-shot-runner` in registry `crates-io` with version `*`
```

The repository has no release tag. The README instead says to clone the source and run `cargo install --path crates/shot-runner`. The live page has no source or release link, so a cold visitor can view the demo but cannot follow the prominent command to use the real CLI.

**Why this fails:** This is the core path from demonstration to the real job. A copyable command is an executable promise, and this one fails immediately. It also conflicts with the README’s source-only installation instructions.

**Concrete fix:** Publish the crate before showing the registry command, or replace it with a tested install path that works now, such as a pinned `cargo install --git … --rev … --locked` command. Add a visible **Open source and install** link. Register an `install-from-clean-machine` claim that runs the exact displayed command in a clean temporary `CARGO_HOME` and asserts `shot-runner --version` and `shot-runner demo` succeed.

### F-3-2 / F-1-3 — BLOCKING — Public claims remain unlisted or are broader than their registered tests

**Location / exact claims:** landing page and README.

| Quote | Gap | Required fix |
| --- | --- | --- |
| **“The receipt records output hashes, frame rate, and colour space.”** / **“A receipt records output hashes, frame rate, and colour space.”** | `run-output-set` checks that a receipt exists; `exact-plan-command` compares argv. No registered claim asserts these three fields and their values. | Add `receipt-metadata`; parse a fresh receipt and compare its hashes, FPS, and colour space with the input and written files. |
| **“No account is needed.”** | `isolated-browser-demo` only checks that the sample page has no password/email control. The unqualified hero statement applies to the CLI/product. | Narrow it to **“The sample asks for no account”**, or register and test the broader CLI claim from a credential-free clean environment. |
| **“Opened pages may still be available from this device.”** | `test:pwa` exercises offline reload, but there is no `claims.json` entry or `@claim:` test for this public status sentence. | Register `offline-opened-pages` and assign a clean service-worker/offline test, or remove the sentence. |
| **“From source, using Rust 1.85 or newer”** | The manifest declares `rust-version = "1.85"`, but no claim test compiles with the minimum supported toolchain. | Add an MSRV claim and a Rust 1.85 build test, or avoid naming an unverified minimum. |
| **“`npm run build` creates the deployable site in `dist/site/`.”** | The quality gate passes, but the public sentence has no claim entry or tagged test. | Register a build-output claim that starts clean and asserts the expected site files in `dist/site/`. |
| **“`cargo package …` creates the publishable crate.”** | `renderer-dependencies` calls `cargo package --list`; it does not own this packaging statement. | Add a package-artifact claim using `npm run pack:cli`, or rewrite this as an unambiguous command instruction without promising publication readiness. |
| **“The factory static work order deploys `main`.”** | This external deployment assertion has no sandbox test and is not useful to a CLI user. | Remove it from the product README or document it in an internal, tested release runbook. |

**Why this fails:** The claims registry is intended to be the complete map from public promise to observable clean-state test. Passing unrelated quality gates does not register these statements. The earlier F-1-3 defect was therefore only partially repaired and is blocking again under the same carried ID.

### F-3-3 — HIGH — Demo terminal labels have 1.10:1 contrast

**Location / quote:** first demo viewport, **“LOCAL TERMINAL / PAPER COURIER”** and **“REAL SAMPLE OUTPUT.”**

**Evidence:** `.terminal-head span` computes to `rgb(18, 18, 16)` at 10 px over the inherited `.terminal-recording` background `rgb(28, 28, 25)`. The WCAG contrast ratio is 1.10:1. The labels are visibly lost against the dark panel at 390 px. The automated axe run reported zero violations, so its pass does not cover this manually verified combination.

**Why this fails:** These labels explain that the first-screen transcript is local and real sample output. Small normal text requires at least 4.5:1.

**Concrete fix:** Set the first recording header text to the existing light paper token (or another measured colour at least 4.5:1 on `#1c1c19`) and add a computed-style contrast regression for the first demo viewport.

### F-3-4 — HIGH — Multiple phone controls are smaller than 44 × 44 px

**Location / evidence:** fresh 390 px contexts, measured browser bounding boxes:

- wordmark/home link on every route: 79 × 25 px;
- header **Demo** and **Home** links: 30 × 44 px;
- header **Terms** link: 37 × 44 px;
- second demo **Copy command** button: 110 × 37 px;
- inline **privacy policy** and **Return home** links: 154 × 23 and 121 × 23 px;
- footer **Demo** and **Terms** links: 34 × 44 and 43 × 44 px.

**Why this fails:** The supplied accessibility and site-structure baseline requires touch targets of at least 44 px in both dimensions. Small targets are easy to miss on the requested phone viewport. Axe does not test this requirement.

**Concrete fix:** Give header/footer links and the wordmark at least 44 px of inline and block hit area, restore a 44 px minimum height on the terminal copy button, and wrap standalone inline links in a 44 px inline-flex target. Add a 390 px test that checks every actionable target’s bounding box.

### F-3-5 — MEDIUM — “Start for real” does not discard demo state

**Location / quote:** demo banner, **“Start for real.”** Source: `site/src/main.js`, which sets `demo:animation-shot-runner:opened`; the link is a plain `href="/"` with no cleanup.

**Evidence:** A fresh context started with a real-data sentinel, entered the demo, reset it, and clicked **Start for real**. The home page retained both:

```json
{
  "animation-shot-runner:real-sentinel": "keep",
  "demo:animation-shot-runner:opened": "true"
}
```

Reset correctly touched only the `demo:` key, and no real data was changed. However, leaving demo mode did not discard its data as required by the demo contract.

**Why this fails:** The action says the visitor is leaving the sandbox, while the sandbox marker remains indefinitely. `.factory/demo.md` documents the current incomplete behavior rather than the required cleanup.

**Concrete fix:** Handle **Start for real** by removing all `demo:animation-shot-runner:*` keys before navigating. Extend `@claim:isolated-browser-demo` to assert demo keys are absent after leaving and real sentinel keys are unchanged.

### F-3-6 — MINOR — The approval concept has two names

**Location / quote:** landing: **“approve the program name”**; README: **“requires an approved executable name.”**

**Why this fails:** “Program name” and “executable name” refer to the same allowlist value. The plain-words rule requires one term for one concept.

**Concrete fix:** Use one phrase in both places. For example: **“Nothing runs until you approve the executable name and add `--yes`.”**

## Copy audit

Counts treat hyphenated compounds and inline command tokens as one word. Commands, headings, controls, and fragments are listed separately so they are not hidden by sentence-only extraction. No sentence exceeds 22 words, and no banned marketing adjective appears.

### Landing-page sentences

| Words | Sentence |
| ---: | --- |
| 7 | Render named animation previews from one command. |
| 13 | For small animation teams and technical artists who need repeatable local preview renders. |
| 4 | Opens the five-shot demo. |
| 5 | It never touches your project. |
| 4 | No account is needed. |
| 6 | Make preview renders you can repeat. |
| 16 | Put shot names, source paths, frame rate, colour space, and command parts in one JSON file. |
| 5 | `plan` shows the exact command. |
| 11 | Nothing runs until you approve the program name and add `--yes`. |
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

Flags from this table are F-3-2 and F-3-6. `JSON`, renderer, frame rate, colour space, cache, and receipt are necessary product terms; receipt is defined on first use. No marketing adjective, metaphor, or mood slogan remains.

### Landing headings, controls, labels, and command copy

| Words | Copy | Check |
| ---: | --- | --- |
| 2 | SR / 01 | Informative wordmark. |
| 3 | LOCAL PREVIEW RUNNER | Names the product class. |
| 5 | Try it with sample data | Result-led required action. |
| 3 | `cargo install animation-shot-runner` | **Blocking: fails; F-3-1.** |
| 2 | Copy command | Result-naming verb. |
| 4 | Runs on local files | Plain fact. |
| 2 | MIT-licensed CLI | Plain fact; registered claim. |
| 3 | HOW IT WORKS | Context label; the following h2 names the outcome. |
| 3 | List your shots | Clear step heading. |
| 3 | Review the command | Clear step heading. |
| 3 | Use the outputs | Clear step heading. |
| 3 | BUNDLED FIVE-SHOT SAMPLE | Clear section label. |
| 4 | Open the sample demo | Result-naming link. |
| 3 | INSTALL THE CLI | Clear section label. |
| 1 | PRIVACY | Clear section label. |

Navigation labels **Demo**, **How it works**, **Privacy**, and **Terms** make sense out of context. The factual proof caption (**“FIVE SAMPLE SHOTS / PAPER COURIER”**, **“24 FPS · sRGB · LOCAL RECEIPTS”**) identifies the displayed artifact rather than adding a slogan.

### README sentences

| Words | Sentence |
| ---: | --- |
| 7 | Render named animation previews from one command. |
| 10 | Shot Runner is for small animation teams and technical artists. |
| 6 | It runs renderer commands you approve. |
| 11 | It writes frames, contact sheets, and JSON receipts on your computer. |
| 10 | A receipt records output hashes, frame rate, and colour space. |
| 10 | Run the five-shot Paper Courier sample after installing from source. |
| 12 | The command copies bundled sample files into a new system temporary folder. |
| 5 | It renders five named shots. |
| 8 | It repeats the run to show cached frames. |
| 9 | It checks a receipt and prints the output folder. |
| 6 | Your project files are not used. |
| 11 | Open `/demo/?demo=1` on the documentation site for the same sample instructions. |
| 10 | The browser demo marker uses a separate `demo:` local-storage key. |
| 13 | Use **Reset demo** to reset it or **Start for real** to return home. |
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
| 9 | `npm run build` creates the deployable site in `dist/site/`. |
| 12 | The factory static work order deploys `main`; do not change infrastructure here. |
| 8 | `cargo package --manifest-path crates/shot-runner/Cargo.toml` creates the publishable crate. |
| 8 | Do not publish the crate from this repository. |
| 7 | The CLI reads and writes local paths. |
| 4 | See Privacy and Terms. |
| 2 | MIT licensed. |
| 4 | See LICENSE and CHANGELOG.md. |

The headings **Shot Runner**, **Try the bundled sample**, **Install**, **Use your own shots**, **Develop and verify**, and **Privacy and license** are meaningful out of context. The fragment **“From source, using Rust 1.85 or newer”** is an unlisted compatibility statement under F-3-2. No README sentence exceeds 22 words. F-3-6 is the only terminology inconsistency; there are no marketing adjectives, metaphor headings, or ambiguous buttons in the README.

## Claim-test results

Every exact command in `.factory/claims.json` was run independently after `npm ci` in the clean clone.

| Claim ID | Result |
| --- | --- |
| `demo-five-shot` | PASS |
| `demo-cache-and-receipt` | PASS |
| `demo-project-isolation` | PASS |
| `review-before-run` | PASS |
| `exact-plan-command` | PASS |
| `run-output-set` | PASS |
| `unchanged-run-cache` | PASS |
| `renderer-dependencies` | PASS |
| `direct-command-expansion` | PASS |
| `relative-paths-and-exit-codes` | PASS |
| `isolated-browser-demo` | PASS |
| `mit-license` | PASS |

No listed test failed. F-3-2 concerns claims absent from the list or copy broader than its assigned listed claim.

## Demo, privacy, and sandbox evidence

- `/?demo=1` redirected to `/demo/?demo=1`; title was **“Demo — Shot Runner”**, with one h1 and the required banner.
- The 390 px first viewport contained the real command transcript and contact sheet. Reset changed only the `demo:` marker and displayed its reset message.
- A caller-directory CLI run left `real-project-sentinel` untouched and printed output under `/tmp/shot-runner-demo-3481-1787964781582` after five renders, five cache hits, and receipt verification.
- Browser request logs for home and demo contained only `https://animation-shot-runner.sociobot.in` requests. No analytics, provider key, CDN script, or third-party font request was present.
- A real-storage sentinel survived demo entry, Reset, and Start for real unchanged. F-3-5 is only about the demo marker not being discarded.

## Structure, links, accessibility, and visual identity

- Live `/`, `/demo/?demo=1`, `/privacy/`, and `/terms/` returned 200. A new unknown URL returned the designed page with HTTP 404.
- Each route has `lang=en`, one h1, one main, route-appropriate title and description, canonical, favicon, 180 px touch icon, OG/Twitter title/description/1200 × 630 image, consistent header, and consistent footer.
- Every discovered same-origin link returned 200, except the intentionally crawled unknown route at 404. In-page `#main`, `#how-it-works`, and `#sample` targets exist. Home → Demo → browser Back restored `/`.
- Live HTML, service worker, JS, CSS, and all four route documents matched the clean candidate build byte for byte by SHA-256.
- Live CSP, framing, referrer, permissions, and cache headers are present. `sw.js` is `no-cache`; hashed assets are immutable.
- `TEST_URL=https://animation-shot-runner.sociobot.in/ npm run test:a11y` reported 39 passes and zero axe violations. Manual checks found F-3-3 and F-3-4, which axe did not cover.
- `npm test`, `npm run build`, and `npm run pack:cli` passed in the clean clone. Initial JS is 1.83 kB raw / 0.88 kB gzip and CSS is 13.74 kB raw / 3.84 kB gzip.
- The warm-paper broadsheet, oversized Instrument Serif typography, crop-mark red, proof rules, and contact-sheet art match `.factory/design.md` and are recognisably product-specific. This is not a generic SaaS template.

## Earlier finding audit

| Earlier ID | Result verified live and in code |
| --- | --- |
| F-1-1 | FIXED — cold mobile and desktop screens state job, audience, and first action. |
| F-1-2 | FIXED — real one-click demo route, first-screen transcript/contact sheet, banner, Reset, isolated `demo:` namespace, and CLI temp run exist. F-3-5 is a narrower exit-cleanup defect. |
| F-1-3 | **NOT FULLY FIXED — carried as blocking F-3-2.** Twelve listed tests pass, but public claims remain unlisted or broader than the registered test. |
| F-1-4 | FIXED — `/demo/` is real and unknown paths return the designed 404 with status 404. |
| F-1-5 | FIXED at its original scope — slogan/metaphor copy is gone and all sentences meet the cap. F-3-6 records the remaining new terminology mismatch. |
| F-1-6 | FIXED — metadata and consistent legal-page skeleton are live. |
| F-1-7 | FIXED — repeated clean `npm test` and PWA checks pass deterministically. |
| F-1-8 | FIXED — undefined paid offer removed. |
| F-2-1 / F-1-2 | FIXED — actual transcript and contact sheet are in the first phone and desktop demo viewport. |
| F-2-2 / F-1-3 | FIXED — assigned cache/receipt test asserts repeat cache hits and tamper failure. |
| F-2-3 / F-1-3 | FIXED — sentinel caller-folder isolation test passes. |
| F-2-4 / F-1-3 | FIXED — expanded planned argv is compared with the recorded argv. |
| F-2-5 / F-1-3 | FIXED — non-demo output set is registered and observed. |
| F-2-6 / F-1-3 | FIXED — non-demo second-run cache reuse is registered and observed. |
| F-2-7 / F-1-3 | FIXED — renderer separation and native no-ffmpeg path are registered and pass. |
| F-2-8 / F-1-3 | FIXED — broad media wording was narrowed to local output behavior. |
| F-2-9 / F-1-3 | FIXED — hostile argv test proves direct expansion without shell interpretation. |
| F-2-10 / F-1-3 | FIXED at its stated scope — relative paths, custom cache behavior, and documented 0/2/3 exits pass. |
| F-2-11 | FIXED — Twitter title, description, and image exist on Demo, Privacy, Terms, and 404. |
| F-2-12 | FIXED — receipt is defined and the exit-code sentence is split. |

All earlier polish, verification, and handoff files were read. Earlier verification defects for relative paths, argv disclosure, cache policy, security headers, sensitive service-worker caching, and axe execution remain fixed in the current live build.

## Missed leverage

The obvious missing capability is a working distribution path, recorded in F-3-1. The brief does not imply an AI-assisted step, and adding one would be decorative. The shot file already provides import/export, while remote sync would conflict with the local-first job; neither warrants a separate feature finding.

## What would make this perfect

1. Make the exact first-screen install command work from a clean machine and test it as a claim.
2. Give every public behavioral statement one complete claim entry and tagged clean-state test.
3. Raise the demo terminal-label contrast to at least 4.5:1.
4. Make every phone target at least 44 × 44 px and test the rendered boxes.
5. Clear the `demo:` namespace on **Start for real** while preserving real keys.
6. Use one term for the approved executable everywhere.

Until all six are complete, the product does not meet the zero-findings release standard.
