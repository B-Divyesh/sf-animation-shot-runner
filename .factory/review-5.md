# Adversarial first-read review 5 — PASS

**Reviewed:** 2026-08-29  
**Candidate:** `64df06e35cb04ddf9fbc148e8b146cfd49df6b6c`  
**Live URL:** <https://animation-shot-runner.sociobot.in/>  
**Viewports:** new Chromium contexts at 390 × 844 and 1440 × 900

## Verdict

**PASS.** This review found zero blocking, high, medium, or minor findings. The landing page, actual bundled CLI sample, claim registry, routes, and prior repairs all held in a clean clone and on the deployed site.

## Cold first read

Before scrolling, both phone and desktop showed:

> **Render named animation previews from one command.**  
> For small animation teams and technical artists who need repeatable local preview renders.  
> **Try it with sample data** — Opens the five-shot demo. It never touches your project.

- **What it does:** Runs named local animation preview renders from a shot file and produces reviewable outputs.
- **For whom:** Small animation teams and technical artists.
- **What to click first:** **Try it with sample data**.

The phone page had no horizontal overflow, no console errors, and kept the primary action, install command, source link, and all three facts inside the first 844 px. The desktop presented the same information without ambiguity.

## Demo and sandbox

`/?demo=1` redirected in one navigation to `/demo/?demo=1`. Its first screen immediately showed the actual `shot-runner demo` command, five rendered sample shots, five cache reuses, receipt verification, and the generated contact sheet. The persistent banner read **“Demo — sample data, nothing is saved”** and exposed working **Reset demo** and **Start for real** controls.

In a fresh browser context, the demo created only `demo:animation-shot-runner:opened`; Reset recreated that key and Start for real removed it before returning home. The request log contained only `animation-shot-runner.sociobot.in` URLs. From a separate temporary caller directory, the shipped CLI printed:

```text
DEMO COMPLETE — 5 sample shots rendered
CACHE CHECK — 5 sample shots reused on repeat
RECEIPT VERIFIED — 2 output files checked
Output: /tmp/shot-runner-demo-…
This folder contains only bundled sample data. Delete it when you are done.
```

This is a real CLI result, not a browser mockup or a manifest validator. The sample is isolated from the caller project.

## Claims and verification

I read `.factory/claims.json` and ran all 18 listed commands independently from a fresh clone. Every tagged claim passed. The full clean-clone chain also passed:

```text
npm ci
npm test
npm run build
TEST_URL=https://animation-shot-runner.sociobot.in npm run test:a11y
TEST_ORIGIN=https://animation-shot-runner.sociobot.in npm run test:browser
npm run pack:cli
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

`npm run build` created `target/release/shot-runner` and `dist/site/`; packaging created a non-empty `animation-shot-runner-0.1.0.crate`. The claims registry covers every claim-like landing and README statement: sample rendering/cache/receipt/isolation, approval, plan/run parity, local output/cache/metadata, renderer requirements, direct argv expansion, relative paths/exits, browser isolation/history/offline behaviour, pinned installation, build/package output, and MIT licensing. No unlisted public claim was found.

## Copy audit

Word counts treat hyphenated terms, file names, and command tokens as one word. There are no sentences above 22 words. There are no banned marketing adjectives, unexplained first-use terms, inconsistent core terms, metaphor headings, or non-result-naming buttons.

### Landing-page sentences

| Words | Sentence |
| ---: | --- |
| 7 | Render named animation previews from one command. |
| 13 | For small animation teams and technical artists who need repeatable local preview renders. |
| 4 | Opens the five-shot demo. |
| 5 | It never touches your project. |
| 6 | The sample asks for no account. |
| 6 | Make preview renders you can repeat. |
| 16 | Put shot names, source paths, frame rate, colour space, and command parts in one JSON file. |
| 5 | `plan` shows the exact command. |
| 11 | Nothing runs until you approve the executable name and add `--yes`. |
| 20 | Each shot gets frames, a contact sheet, and a JSON receipt that records output hashes, frame rate, and colour space. |
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
| 7 | Local animation previews with JSON output records. |
| 3 | You are offline. |
| 9 | Opened pages may still be available from this device. |

Headings name the job or a usable section: **How it works**, **Run the sample in a temporary folder**, **Start with your own shot file**, and **Write preview output locally**. Controls name their results: **Try it with sample data**, **Copy command**, **Open source and install on GitHub**, and **Open the sample demo**. No copy finding.

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

Terminology remains consistent: **shot file**, **plan**, **executable name**, **preview**, **local cache**, **receipt**, and **sample demo**. `receipt` is defined before later use on Home, Demo, and README.

## Structure, privacy, accessibility, and visual identity

- Home, Demo, Privacy, Terms, and missing-route checks returned the correct 200/404 status. Each had one h1, one main landmark, its required route title, description, canonical, Open Graph/Twitter image metadata, favicon, and touch icon.
- The missing route returned the designed **Page not found — Shot Runner** page at HTTP 404, with a useful home action.
- Header/footer links were checked on every route. Internal links resolved successfully; the GitHub install link resolved successfully. No dead links were found.
- Browser checks confirmed heading focus and polite route announcement on navigation. Back and Forward restored the saved scroll/focus state.
- Live axe checks passed across all five routes. The browser quality suite found zero console errors, horizontal overflow, target-size failures, or focus failures. The demo terminal text meets the required contrast.
- The PWA claim passed after service-worker control and offline reload. The live request log was same-origin-only for the demo flow; no third-party font/script/service request appeared.
- The warm-newsprint, carbon-ink, contact-sheet identity matches `.factory/design.md`. The reviewed original art is an editorial five-frame proof, not a generic SaaS hero/card template. Self-hosted Instrument Serif and IBM Plex Mono support the design.
- No AI capability is implied by the brief. An AI feature would be decorative here; no missed AI leverage finding applies.

## Earlier findings checked from scratch

Every earlier review, polish report, and handoff was read. The following confirmations are based on current live behaviour and source/tests, not the prior status labels.

| Earlier finding IDs | Current confirmation |
| --- | --- |
| F-1-1, F-1-5, F-4-3 | Cold first screen names job, audience, and first action; plain copy, useful headings/actions, and full **Shot Runner** wordmark remain live. |
| F-1-2, F-2-1, F-2-3, F-3-5 | One-click browser demo, banner/reset/exit, first-viewport real output, temporary CLI folder, and namespace cleanup all work. |
| F-1-3 and F-1-3a–F-1-3ak; F-2-2–F-2-10; F-3-2 | The 18-claim registry and tagged clean-state tests cover retained public claims. All exact commands passed independently. |
| F-1-4, F-4-2 | Real document routes, designed HTTP 404, route focus announcement, and Back/Forward scroll/focus restoration work. |
| F-1-6, F-2-11 | Route metadata and the shared header/footer skeleton remain complete. |
| F-1-7 | The complete test chain, PWA test, and repeated build succeeded from the clean clone. |
| F-1-8 | No paid offer, checkout, unspecified entitlement, or future-delivery claim is present. |
| F-2-12, F-4-1 | `receipt` is defined before first meaningful use on the current Home and Demo routes. |
| F-3-1 | The visible pinned Git installation command is the tested clean-machine path. |
| F-3-3, F-3-4, F-3-6 | Demo contrast, 44 px phone controls, and the single **executable name** approval term remain correct. |

No prior finding is unfixed, half-fixed, or regressed.

## What would make this perfect

Nothing is required for acceptance in this round. Keep the pinned install revision, bundled sample, claim tests, and browser-quality checks current whenever the CLI or landing copy changes; any new public outcome should be added to `claims.json` with a clean-state observable test before release.
