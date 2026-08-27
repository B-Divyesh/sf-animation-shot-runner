# Independent verification 1 — FAIL

**Verified:** 2026-08-27  
**Candidate commit:** `b3b78bd8a3692e4b3f867a457587d09b9bb6b9bd`  
**Live URL:** `https://animation-shot-runner.sociobot.in/`

## Verdict

**FAIL — do not release this candidate.** The documented, normal CLI invocation with a relative manifest path cannot execute a renderer. The paid-license service worker also caches bearer-like license tokens and verification responses indefinitely, defeating revocation checks.

The static live deployment is otherwise the exact output of the tested candidate: local and live SHA-256 values match for `index.html` (`8fafd51f3033bb9c6ff8be777613a0352838ee8f5d8291f4a4d4fb4d828ddc05`), `assets/main-DNe3Gmpc.js` (`861048f719978c99ec4cb6efc1c9ad57fe7172448bbda0da06be1c3cdd58f6b0`), and `assets/main-mMgJoWz-.css` (`fa26bdf88bd383c03e7ace6249ffaccf3d88a59b090de2ea2387d8d5e7980b48`).

## Clean-checkout gates

| Check | Result | Evidence |
| --- | --- | --- |
| `npm ci` | PASS | 21 packages audited; 0 vulnerabilities. |
| `npm test` | PASS | 5 Rust tests passed; site contract checks passed. |
| `cargo fmt --all -- --check` | PASS | Clean. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Clean. |
| `npm run build` | PASS | Release CLI and `dist/site/` built. |
| `npm run pack:cli` | PASS | `cargo package` built and verified `animation-shot-runner v0.1.0`. |
| `npm run test:a11y` from an initially clean checkout | FAIL | Script tried to write ignored `.factory/evidence/axe.json` but does not create its parent directory (`ENOENT`). After `mkdir -p .factory/evidence`, it passes: 40 axe passes, 0 violations / 0 serious or critical. |

Build output is within declared static budgets: initial JS 6,302 bytes, CSS 10,303 bytes, fonts 35,740 bytes total, mobile hero 29,572 bytes, desktop hero 148,544 bytes. An attempted Lighthouse run recorded 100/100/100/100 and FCP 1.0 s, LCP 1.5 s, CLS 0, TBT 70 ms, but its Chrome tab then crashed (`TARGET_CRASHED`); those scores are therefore indicative only, not a clean Lighthouse pass.

## CLI / packed-consumer exercise

Installed the packaged crate into a fresh `/tmp/shot-runner-consumer.*` consumer with `cargo install --path target/package/animation-shot-runner-0.1.0 --root <consumer>/install`, then used only that installed `shot-runner` binary.

- `--help` described the single binary, four commands, `--json`, and version.
- `init` wrote a manifest and correctly refused overwrite (exit 2).
- `plan --json` accepted a five-shot manifest; `run` correctly rejected omitted `--yes` and a non-matching allowlist (both exit 3); a parent-directory output manifest was rejected (exit 2).
- With an **absolute** manifest path, one command rendered five named shots (`rendered: 5`), wrote five receipts/contact sheets, and all five receipts verified. A repeat had `rendered: 0, cache_hits: 5`. Mutating one source re-rendered only that selected shot. Tampering a copied frame made `verify` return exit 5; re-running the shot restored it and `verify` passed.
- With the README and web-demo’s documented normal command from its project directory, `shot-runner --json run shots.json --allow-command cp --yes` failed before execution with exit 4: `could not start "cp" ... No such file or directory (os error 2)`. `Path::parent()` for `shots.json` is an empty path and is passed to `Command::current_dir`, which is invalid. This also breaks the documented Blender command and the core job-to-be-done for ordinary relative manifests.

## Site, accessibility, PWA, and privacy exercise

- Local production build and live URL: desktop and 390 px mobile both planned the five-shot sample, displayed malformed JSON recovery guidance, accepted a boundary `fps: 0.001`, had no horizontal overflow, and had zero page or console errors. No outbound requests occurred before an optional license action.
- Keyboard-only live testing tabbed through skip link, navigation, textarea, plan button, checkout and restore controls; each inspected focus state had a visible `3px solid` outline. Reduced-motion mode yielded `0s` transition and animation duration.
- Axe on both local production build and live URL: 40 passes, zero violations, zero serious/critical findings.
- Live service worker registered, `registration.update()` resolved with an active worker, and a 390 px offline reload displayed the cached page and offline notice. The deployment includes `skipWaiting` and cache-version cleanup, but a true version-change rollout was not induced without changing the candidate.
- Source audit found no CLI network code, analytics, telemetry, CDN fonts/scripts, `eval`, or media upload. The optional site license flow is the only runtime API call. Privacy/terms routes return 200.

## Defects

### P0 — documented relative manifest execution is broken

**Reproduction:** in a directory containing `shots.json`, run the documented `shot-runner run shots.json --allow-command blender --yes` (a harmless `cp` renderer was used in verification).  
**Actual:** exit 4 before renderer launch: `could not start "cp" ... No such file or directory (os error 2)`.  
**Expected:** renderer runs from the manifest directory, then frames/contact sheet/receipt are written.  
**Impact:** the primary README and browser-demo command cannot perform the real job with normal relative paths. Absolute manifest paths work only as a workaround.

### P1 — service worker persists license tokens and verification responses, and makes revocation stale

**Reproduction:** register the live worker, then visit `/?license=verification-probe-token`. Cache Storage contains both `https://animation-shot-runner.sociobot.in/?license=verification-probe-token` and `https://api.sociobot.in/api/v1/products/animation-shot-runner/verify?license=verification-probe-token`.  
**Impact:** the generic cache-first fetch handler caches every GET. It persists tokens in URL-keyed Cache Storage and will return a previously valid verification response instead of contacting Sociobot after the daily recheck, so expired/revoked licenses can remain unlocked. This conflicts with the paid-unlock and privacy requirements.

### P2 — deployed caching does not honor the declared immutable asset policy

The candidate ships `site/public/_headers` with one-year immutable directives, but live JS, WebP, font, and HTML responses all return `cache-control: public, must-revalidate, max-age=30`. The site works offline through its service worker, but browser/CDN cache policy is not the stated long-lived hashed-asset policy.

### P2 — no clean a11y script invocation

`npm run test:a11y` fails in a clean checkout because `.factory/evidence/` is ignored and absent. The script must create its output directory or avoid requiring an untracked directory.

### P3 — header hardening is incomplete

Live responses have HSTS, referrer policy, and `X-Content-Type-Options`, but no CSP, `frame-ancestors`/`X-Frame-Options`, Permissions-Policy, or COOP header. This did not produce an axe or functional failure, but CSP/frame protection is particularly appropriate for a site accepting a license token in its URL.

## Required next steps

1. Treat an empty manifest parent as `.` (and add a CLI integration test using `shots.json` relative to CWD).
2. Restrict the service-worker cache to same-origin static shell assets; never cache API responses, navigations with `license` query parameters, or arbitrary GETs. Re-test revocation after a cached valid verdict.
3. Make the deployment apply immutable cache headers to hashed/static assets and add appropriate security headers.
4. Make `test:a11y` self-contained, then rerun the full clean-checkout verification and publish a new candidate.
