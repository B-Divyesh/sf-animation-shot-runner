# Adversarial first-read review 1 — FAIL

**Reviewed:** 2026-08-28  
**Candidate:** `d3ee225420a14ab104da42532e8b377fdb4be613`  
**Live URL:** <https://animation-shot-runner.sociobot.in/>  
**Viewports:** fresh Chromium contexts at 390 × 844 and 1440 × 900

## Verdict

**FAIL.** There are blocking defects. A first-time phone visitor cannot tell what job the tool performs, for whom, or what result-producing action to take first. The product also lacks the required one-click, isolated CLI demo and has no claims registry or claim tests. The live `/demo` and unknown URLs silently return the landing page rather than a demo or designed 404.

## Cold first read

Before scrolling, the page showed the headline **“Every shot. One receipt.”**, a command to install the package, and **“Read the 3-minute setup.”**

- **What does it do?** Not answerable with confidence. “Receipt,” “proof,” and “previews” do not say that it runs approved local renderer commands from a shot manifest and creates cached previews/contact sheets.
- **For whom?** Not answerable. The hero does not name small animation teams or technical artists.
- **What should I click first?** Not answerable. The visible choices are **“Copy”** (an installation command) and **“Read the 3-minute setup.”** Neither names a result or lets a visitor try the product. The mobile nav calls the validator **“TRY MANIFEST,”** but it is below the fold and is not a demo run.

This fails the mandatory first-screen gate. The exact text that fails it is **“Every shot. One receipt.”** It is a slogan, not the job; **“Run five—or fifty—named previews without reopening a browser.”** omits both the user and the essential local CLI/rendering context.

## Findings

### F-1-1 — BLOCKING — First screen does not state the job, user, or first action

**Location / quote:** landing hero, 390 px and desktop: **“Every shot. One receipt.”**; **“Run five—or fifty—named previews without reopening a browser.”**; **“cargo install animation-shot-runner”**; **“Read the 3-minute setup.”**

**Why this fails:** “Receipt” and “proof” are product lore. They do not establish that this is a local command-line runner for repeatable animation preview renders. The named audience from the brief is absent. The only above-the-fold actions are an installation command and a setup link, so a visitor cannot try the product in 30 seconds.

**Concrete fix:** Replace the hero with, for example:

> **Render named animation previews from one command**  
> For small animation teams and technical artists who need repeatable local preview renders.  
> **Try it with sample data** — runs five bundled sample shots in a temporary folder.  
> Local files only · No account · MIT core CLI

Make the sample action the primary 48 px button next to the install command. Keep the broadsheet visual system, but remove decorative labels from the first-read path.

### F-1-2 — BLOCKING — No one-click sample demo or isolated demo sandbox

**Location / quote:** landing has no **“Try it with sample data”** action. The closest feature is `#demo`, headed **“Proof your run before it runs.”** It says **“This browser demo parses only what you type. Nothing is sent or executed.”**

**Evidence:** A clean `?demo=1` browser context displayed the normal landing page with the browser validator prefilled. It had no **“Demo — sample data, nothing is saved”** banner, **“Reset demo”**, or **“Start for real”** control; local/session storage remained empty because this is not a sandboxed product run. Clicking **“Validate plan”** produced five `HELD` rows and explicitly said **“Nothing ran.”** `shot-runner demo` exits with `error: unrecognized subcommand 'demo'`. There is no `examples/` directory or `.factory/demo.md`.

**Why this fails:** The artifact class is CLI. A manifest parser in a webpage neither runs the main job nor shows a contact sheet/receipt/cache result. A visitor cannot run the promised five-shot workflow with bundled sample data. `/demo` is not a direct demo route.

**Concrete fix:** Ship `examples/` containing a realistic five-shot, harmless renderer fixture and implement `shot-runner demo` (or `shot-runner --demo`) to copy it into a new temp directory, run the same pipeline, print the output location, and leave the user’s data untouched. Make `/demo` a real page with a self-hosted terminal recording of that exact command and the resulting contact sheet/receipt. Use an explicit `demo:` storage namespace only if browser state is needed. Show the persistent banner, Reset, and Start-for-real controls required by the demo contract; document all of it in `.factory/demo.md` and test it from a temp directory.

### F-1-3 — BLOCKING — Claims cannot be verified; every live claim is unlisted

**Location / quote:** `.factory/claims.json` does not exist. The landing page and README make the claim-like statements listed in Appendix C, including **“ZERO UPLOADS”**, **“The demo never executes them.”**, **“The CLI stays complete and MIT licensed.”**, **“Media never leaves your machine.”**, and **“Shot Runner has no telemetry and no network code.”**

**Evidence:** `find .factory -name claims.json` returned no file. Therefore there are no listed claim tests to run from a clean clone. The fresh-clone suite was run at the candidate commit (`npm ci`, `npm test`, `npm run build`, `npm run test:a11y`, `npm run pack:cli`); it does not define or execute any `@claim:` tests. A fresh browser request log for normal and `?demo=1` visits contained only same-origin assets, but that one observation cannot substitute for claimed, reproducible coverage.

**Why this fails:** These are facts a visitor can rely on. With no claims inventory, no sandbox instructions, and no tagged observable tests, none is release-verifiable. This is expressly a blocking condition in the review contract.

**Concrete fix:** Add `.factory/claims.json` and one clean-state test tagged `@claim:<id>` for each retained claim. At minimum test: bundled demo renders five named shots and writes a contact sheet/receipt; repeat run uses the cache; plan/run confirmation boundary; receipt tamper detection; no CLI network requests; and browser demo request log/storage isolation. Delete or narrow claims that cannot be tested. Append the actual test command, sandbox, and evidence path per claim.

### F-1-4 — BLOCKING — `/demo` and unknown routes are the landing page, not real routes

**Location / evidence:** live `GET /demo`, `GET /404`, and `GET /missing-review-probe` each returned **200** with title **“Shot Runner — repeatable animation preview renders”** and h1 **“Every shot. One receipt.”**

**Why this fails:** `/demo` neither opens a demo nor preserves a demo state. An unknown address pretends to be the home page rather than explaining it cannot be found. This fails the required direct demo URL, designed 404, and truthful routing behavior; back/forward and route-change focus/announcement cannot be checked for non-existent application routes.

**Concrete fix:** Add a real `/demo/` document/route with title **“Demo — Shot Runner”** and its own h1. Add a designed `/404.html` with title **“Page not found — Shot Runner”**, one useful h1, and a Home link; configure Static Web Apps `responseOverrides` so missing paths get an actual 404 response. Where client-side navigation remains, implement history restoration, focus transfer to the new h1, and an `aria-live` announcement.

### F-1-5 — HIGH — Copy is not first-read plain language

**Location / quotes:** The headings **“Every shot. One receipt.”**, **“A tiny layer between ‘ready’ and ‘review it.’”**, **“Proof your run before it runs.”**, **“No daemon. No account. No render farm.”**, **“Keep the runner. Add the field guide.”**, and **“Your production desk is unlocked.”** are contextless slogan/metaphor headings. The page also relies on unexplained terms such as **“provenance,” “proof,” “inert,” “allowlist,” “daemon,”** and **“content cache.”**

**Why this fails:** A distracted new visitor cannot infer a useful section purpose from these headings. Several buttons and links describe navigation or mechanics rather than a result: **“Copy,” “Read the 3-minute setup,” “TRY MANIFEST,”** and **“Have a license? Restore it.”** The comprehensive sentence audit is in Appendix A; the flagged long and jargon-heavy README sentences are in Appendix B.

**Concrete fix:** Use section names and result verbs: **“How Shot Runner renders previews,” “Try the five-shot sample,” “Install the CLI,” “Optional Producer Toolkit,”** and **“Restore your Producer Toolkit license.”** Replace **“Validate plan”** with **“Show sample render plan”** in demo mode. Split every Appendix B long sentence into the proposed short statements, and define unfamiliar command words on first use.

### F-1-6 — HIGH — Required metadata and consistent legal-page skeleton are incomplete

**Location / evidence:** The home, Privacy, and Terms documents have titles, descriptions, one h1, `lang`, and an SVG favicon. None has a canonical link, Open Graph/Twitter metadata, or 180 px Apple touch icon. Legal pages omit the footer product one-liner, “Built by Param Factory,” build/version id, and one of the required Privacy/Terms footer links (Privacy only links Terms; Terms only links Privacy). Their headers also omit the consistent Demo/Privacy navigation.

**Why this fails:** Shared routes do not have the specified, consistent product skeleton and cannot produce a correct share preview. The legal pages are visually and structurally partial versions of the landing page.

**Concrete fix:** Add route-specific canonical, OG title/description/image, Twitter card, and Apple touch icon tags. Use the generated product proof to produce a real 1200 × 630 social image. Share one header/footer component or template that includes Home, Demo, Privacy, Terms, product one-liner, “Built by Param Factory,” and a build id on all routes.

### F-1-7 — MEDIUM — The test quality gate is flaky

**Location / evidence:** In the initial clean dependency install in this sandbox, `npm test` failed in `site/tests/pwa-security.mjs` at `page.goto(.../?license=browser-security-probe)` with `net::ERR_CONNECTION_REFUSED` after the local Vite preview was started. An immediate retry of `npm run test:pwa` passed, as did the eventual fresh-clone chained suite. The test can therefore report a failed quality gate without a product change.

**Why this fails:** A release gate must be reliably repeatable. The verifier cannot distinguish a product failure from a preview-server lifecycle race.

**Concrete fix:** Make the test wait for and retain the preview process through every navigation, emit its stderr on failure, and use `try/finally` to close browser/server deterministically. Run it repeatedly in CI or add a regression that proves the server remains reachable after service-worker control/reload.

### F-1-8 — MEDIUM — The paid offer promises unavailable or undefined deliverables

**Location / quote:** **“The toolkit unlocks production manifest recipes, handoff checklists, and team receipt conventions on this page—plus future toolkit updates.”** The unlocked state says **“Toolkit recipe downloads will appear here as they ship.”**

**Why this fails:** No recipe, checklist, convention, or download exists in the repository or unlocked UI. A buyer cannot inspect what is delivered now, and “as they ship” leaves the paid result undefined. These are also unlisted claims under F-1-3.

**Concrete fix:** Either ship the named toolkit materials and list the exact files/versions available immediately after purchase, or remove the paid offer until it has a defined current deliverable. Do not sell unspecified future content.

## Demo, privacy, accessibility, and history checks

- **Demo sandbox:** Failed as described in F-1-2. The browser validator made only same-origin requests and did not write storage in the sampled run, but it is not the required CLI demo and has no demo namespace/banner/reset.
- **Privacy/offline:** Normal live-page and `?demo=1` request logs were same-origin only. The optional license UI contains the only Sociobot call in source. A controlled 390 px offline reload returned 200 from the service worker and showed the offline notice. These observations do not validate the unregistered public claims.
- **Accessibility:** `TEST_URL=https://animation-shot-runner.sociobot.in/ npm run test:a11y` passed: 40 axe passes, 0 violations, 0 serious/critical. Fresh desktop and phone contexts had no page/console errors, no horizontal overflow, and visible focus styling was present. This does not cure the content, demo, or routing failures.
- **Earlier reports:** No earlier `.factory/review-*.md` or `.factory/polish-*.md` exists. I read `.factory/verification-1.md`, `verification-2.md`, `verification-3.md`, and the prior handoff. The earlier relative-manifest, command-argument disclosure, license-cache, cache-header, header-hardening, and clean-a11y-directory findings are confirmed fixed: the current tests include the relevant CLI/PWA coverage; live hashed assets are immutable, `sw.js` is `no-cache`, headers include CSP/framing protections; and live axe passes. They are not re-filed as regressed findings.
- **Links:** Crawled live internal footer/legal links and the GitHub source; all returned 200. The checkout endpoint returned 200 and redirected to Dodo. No dead-link finding.
- **Missed leverage / AI:** The brief does not imply an AI step. The obvious missing capability is the mandated sample CLI run, captured in F-1-2; an AI feature would be decorative here.
- **Visual identity:** The warm-paper, mono/serif broadsheet system is distinct and matches `.factory/design.md`; it is not filed as a generic-template defect.

## Appendix A — Landing copy sentence audit

Word count treats contractions and hyphenated compounds as one word. Commands, labels, and headings are audited separately below; dynamic planned-shot output is not a landing sentence.

| Words | Landing sentence |
| ---: | --- |
| 10 | Run five—or fifty—named previews without reopening a browser. |
| 16 | Shot Runner turns local renderer commands into cached frames, contact sheets, and provenance you can inspect. |
| 14 | Keep renderer commands, source paths, FPS, and colorspace together in one reviewable JSON manifest. |
| 3 | `plan` is inert. |
| 9 | `run` requires both an exact executable allowlist and `--yes`. |
| 6 | Share a contact sheet and receipt. |
| 11 | Re-run unchanged sources from the content cache; verify every output hash. |
| 8 | This browser demo parses only what you type. |
| 5 | Nothing is sent or executed. |
| 10 | Validate the manifest to see which commands would be allowed. |
| 5 | The demo never executes them. |
| 8 | Install your renderer separately and observe its license. |
| 12 | Shot Runner itself does not require ffmpeg unless your chosen command does. |
| 7 | The CLI stays complete and MIT licensed. |
| 19 | The toolkit unlocks production manifest recipes, handoff checklists, and team receipt conventions on this page—plus future toolkit updates. |
| 5 | One purchase; no render-minute billing. |
| 9 | Toolkit recipe downloads will appear here as they ship. |
| 7 | Your permanent core CLI access is unchanged. |
| 6 | Sociobot / Dodo is merchant of record. |
| 8 | Refunds are handled there and revoke the license. |
| 6 | Local previews with a paper trail. |
| 2 | You’re offline. |
| 11 | The docs still work; license verification will retry when you reconnect. |

Standalone landing copy that fails the heading/button test: **“LOCAL PIPELINE EDITION,” “Every shot. One receipt.,” “A tiny layer between ‘ready’ and ‘review it.’,” “Proof your run before it runs.,” “No daemon. No account. No render farm.,” “Keep the runner. Add the field guide.,” “Your production desk is unlocked.,” “Copy,” “Read the 3-minute setup,”** and **“TRY MANIFEST.”** Replace them as specified in F-1-1 and F-1-5.

## Appendix B — README copy sentence audit

| Words | README sentence |
| ---: | --- |
| 15 | Shot Runner is a local, manifest-driven preview renderer for small animation teams and technical artists. |
| 30 | It runs the renderer commands you explicitly approve, caches frame sequences by source content, makes contact sheets, and writes a portable JSON receipt containing hashes, FPS, colorspace, command, and outputs. |
| 5 | Media never leaves your machine. |
| 7 | Prebuilt binaries will be attached to releases. |
| 5 | From source (Rust 1.85+): |
| 6 | Shot Runner does not bundle renderers. |
| 20 | Install and license Blender, Motion Canvas, ffmpeg, or any other command used by your manifest according to that tool’s terms. |
| 19 | ffmpeg is only required if your own renderer command uses it; native PNG/JPEG contact sheets need no ffmpeg. |
| 4 | Create a starter manifest: |
| 10 | A manifest names each shot and gives a tokenized command. |
| 11 | Placeholders are expanded without a shell: `{source}`, `{frames}`, `{shot}`, and `{cache}`. |
| 8 | Inspect first, then run with both trust gates: |
| 4 | `plan` never executes commands. |
| 33 | It prints both the manifest token vector and the exact expanded `run argv` vector that `run` will pass to the renderer, including every argument and the resolved `{source}`, `{frames}`, `{shot}`, and `{cache}` values. |
| 9 | JSON plan output exposes these as `command` and `argv`. |
| 11 | Review the complete `run argv` for every shot before passing `--yes`. |
| 12 | The expansion is a snapshot of the source content and cache location. |
| 8 | If either changes after review, run `plan` again. |
| 19 | When using a custom cache location, pass the same option to both commands so the reviewed vector stays identical: |
| 27 | `run` refuses executable names that are not explicitly allowlisted and refuses to run without `--yes`; this is deliberate protection against untrusted manifests and works predictably in CI. |
| 18 | A successful run writes `contact-sheet.png`, a copied frame sequence, and `receipt.json` under the manifest’s output directory. |
| 8 | Re-running unchanged inputs uses the content-addressed local cache. |
| 13 | The receipt records the exact argv that ran, making review/execution parity auditable. |
| 23 | For `shot-runner run shots.json`, the manifest directory is the current directory: relative sources, output, cache, and renderer working directory all resolve there. |
| 37 | This is also true when the manifest is in a named relative directory such as `project/shots.json`; displayed command paths are resolved to absolute paths so the renderer sees the same files from that working directory. |
| 19 | Exit codes: `0` success, `2` manifest/usage error, `3` trust denied, `4` renderer failed, `5` output or verification failed. |
| 8 | `npm test` runs Rust tests and website checks. |
| 11 | `npm run build` creates the deployable documentation site at `dist/site/`. |
| 17 | `cargo package --manifest-path crates/shot-runner/Cargo.toml` creates the publishable Rust crate; the factory owns publication credentials. |
| 9 | Shot Runner has no telemetry and no network code. |
| 7 | Sources, frames, caches, and receipts stay local. |
| 27 | The optional Producer Toolkit license is restored and verified by the documentation site through Sociobot; it does not gate the core renderer, accessibility, safety, or receipt export. |
| 2 | MIT licensed. |
| 7 | See `site/privacy/index.html` and `site/terms/index.html`. |
| 5 | See `LICENSE` and `CHANGELOG.md`. |

**Flags and precise rewrites:**

- **F-1-5a:** The 30-word opening sentence exceeds 22 words and packs five outcomes. Rewrite: “It runs renderer commands you approve. It caches frame sequences and creates contact sheets. It writes a JSON receipt with hashes, FPS, colorspace, command, and outputs.”
- **F-1-5b:** The 33-word `plan` sentence exceeds 22 words and uses dense jargon. Rewrite: “`plan` shows the manifest command and the exact command that `run` will execute. It resolves every placeholder before anything runs.”
- **F-1-5c:** The 27-word `run` sentence exceeds 22 words. Rewrite: “`run` accepts only executable names you allow. It also requires `--yes`. These checks stop accidental execution of an untrusted manifest.”
- **F-1-5d:** The 23-word relative-path sentence exceeds 22 words. Rewrite: “With `shot-runner run shots.json`, the manifest directory is the working directory. Sources, output, cache, and renderer paths resolve there.”
- **F-1-5e:** The 37-word named-relative-path sentence exceeds 22 words. Rewrite: “The same rule applies to `project/shots.json`. Displayed command paths are absolute so the renderer sees the reviewed paths.”
- **F-1-5f:** The 27-word Producer Toolkit sentence exceeds 22 words and makes unverified promises. Rewrite only after tests/materials exist: “The optional site license is checked through Sociobot. It never changes access to the MIT CLI.”
- **F-1-5g:** `manifest-driven`, `tokenized`, `argv`, `inert`, `allowlist`, `content-addressed`, and `auditable` are unexplained specialist terms. Define each on first use or replace them with “shot file,” “space-separated command parts,” “does not run anything,” “approved program name,” “cache based on source contents,” and “can be checked later.”

## Appendix C — Unlisted claims

Each row is an independent unlisted-claim finding under F-1-3 because no `.factory/claims.json` entry/test exists. “Location” is live landing unless marked README.

| ID | Quote / location | Observable test needed |
| --- | --- | --- |
| F-1-3a | “Run five—or fifty—named previews…” | bundled demo renders five named shots |
| F-1-3b | “cached frames, contact sheets, and provenance…” | demo writes cache, sheet, receipt with expected fields |
| F-1-3c | “ALLOWLISTED COMMANDS / CONTENT CACHE / CONTACT SHEETS / JSON RECEIPTS / ZERO UPLOADS” | one observable test per capability; request-log test for uploads |
| F-1-3d | “`plan` is inert.” | plan fixture asserts no child process/output |
| F-1-3e | “`run` requires … allowlist and `--yes`.” | omitted/invalid trust-gate tests |
| F-1-3f | “verify every output hash.” | tampered receipt/frame fixture fails |
| F-1-3g | “Nothing is sent or executed.” | browser request log plus no-execution fixture |
| F-1-3h | “The demo never executes them.” | demo temp-dir/no-child-process test |
| F-1-3i | “does not require ffmpeg…” | fixture using native image paths without ffmpeg |
| F-1-3j | “The CLI stays complete and MIT licensed.” | entitlement-independent core CLI test; license assertion |
| F-1-3k | “One purchase; no render-minute billing.” | documented billing contract test or delete |
| F-1-3l | “Refunds … revoke the license.” | recorded entitlement fixture proves revoked state locks |
| F-1-3m | “NO CLOUD MEDIA / NO TELEMETRY” | CLI and browser whole-flow outgoing-request test |
| F-1-3n | “The docs still work…” offline notice | cached offline demo/documentation test |
| F-1-3o | README: “Media never leaves your machine.” | CLI network-denial/request-log test |
| F-1-3p | README: “Placeholders are expanded without a shell.” | hostile placeholder test proves no shell execution |
| F-1-3q | README: “A successful run writes…” | demo asserts exact outputs |
| F-1-3r | README: “Re-running unchanged inputs uses…” | two-run cache-hit test |
| F-1-3s | README: “no telemetry and no network code.” | static/network behavior test |
| F-1-3t | paid Toolkit deliverables and updates | entitlement fixture plus shipped-artifact test, or delete |
| F-1-3u | “Keep renderer commands, source paths, FPS, and colorspace together…” | planned fixture exposes each field |
| F-1-3v | “Share a contact sheet and receipt.” | sample run produces both files |
| F-1-3w | “This browser demo parses only what you type.” | browser fixture asserts only supplied manifest is parsed |
| F-1-3x | “Validate … which commands would be allowed.” | manifest-validator fixture asserts allowlist/result |
| F-1-3y | “Toolkit recipe downloads will appear here as they ship.” | current download artifact test, or delete |
| F-1-3z | “Your permanent core CLI access is unchanged.” | license-state fixture runs core CLI locked/unlocked |
| F-1-3aa | “Sociobot / Dodo is merchant of record.” | documented billing integration contract, or delete |
| F-1-3ab | README: “Prebuilt binaries will be attached to releases.” | release artifact check, or delete until true |
| F-1-3ac | README: “Shot Runner does not bundle renderers.” | package-content assertion |
| F-1-3ad | README: “A manifest names each shot…” | parser fixture with named shots |
| F-1-3ae | README: “JSON plan output exposes these as `command` and `argv`.” | JSON-plan schema assertion |
| F-1-3af | README: “The expansion is a snapshot…” and reviewed vectors stay identical | mutation/custom-cache fixture |
| F-1-3ag | README: relative sources/output/cache/renderer paths resolve in the manifest directory | relative-manifest integration fixture |
| F-1-3ah | README: documented exit-code mapping | one fixture per listed exit condition |
| F-1-3ai | README: `npm test` / build / package outcome claims | clean-clone CI test |
| F-1-3aj | README: sources, frames, caches, and receipts stay local | CLI whole-flow filesystem/request-log test |
| F-1-3ak | README: Producer Toolkit does not gate core renderer/accessibility/safety/receipt export | locked-entitlement core-flow and site test |

## What would make this perfect

1. A visitor lands on a plain job headline, names the intended animation team, and runs a safe five-shot sample in one click/command without setup.
2. The sample visibly produces the real outputs: five previews, a contact sheet, a cache hit on repeat, and a receipt that catches a tampered frame.
3. Every retained claim has one independently runnable clean-state test and no sentence promises an unspecified future paid deliverable.
4. `/demo`, legal routes, and a designed 404 have complete route metadata and the same polished product skeleton.
5. The broadsheet art direction remains, but its copy becomes direct enough that a phone visitor understands it before the first scroll.
