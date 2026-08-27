# Independent verification 2 — FAIL

**Verified:** 2026-08-27  
**Candidate commit:** `8616729fa7190925bb42306f73dbd80f8aee5eb7` (`fix: secure relative runs and license cache`)  
**Live URL:** `https://animation-shot-runner.sociobot.in/`

## Verdict

**FAIL — do not release this candidate.** The two failures from verification 1 are fixed, but the CLI's safety confirmation is not informed: `plan` hides every command argument while `run --yes --allow-command <executable>` executes those hidden arguments. An untrusted manifest can therefore attach arbitrary flags and paths to an executable the operator has allowlisted; the documented review step gives the operator no way to inspect them. This fails the brief's requirement never to execute untrusted manifest commands without meaningful confirmation.

The deployed documentation/PWA is otherwise the exact static output of this candidate and passed the functional, privacy, accessibility, offline, and responsive checks recorded below.

## Clean-checkout gates

| Check | Result | Fresh evidence |
| --- | --- | --- |
| Candidate / worktree | PASS | Clean checkout at exactly `8616729fa7190925bb42306f73dbd80f8aee5eb7` before verifier documentation. |
| `npm ci` | PASS | Installed 20 packages; npm audited 21 packages and found 0 vulnerabilities. |
| `npm test` | PASS | 5 Rust unit tests, 1 relative-manifest integration test, site contracts, and production-build PWA Cache Storage regression passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Clean. |
| `cargo fmt --all -- --check` | PASS | Clean. |
| `npm run build` | PASS | Release binary and `dist/site/` built. Bundles: JS 6,302 B, CSS 10,303 B. |
| `npm run pack:cli` | PASS | `cargo package --allow-dirty` packaged and verified `animation-shot-runner v0.1.0`. |
| `npm run test:a11y` | PASS | 40 axe passes, 0 violations, 0 serious/critical. |

An attempted Lighthouse 13.4.1 mobile run against the local production server could not complete because its Chrome tab crashed. This is a QA-environment limitation, not treated as a product score. Bundle budgets are independently within limits: fonts total 35,740 B, mobile proof image 29,572 B, desktop proof image 148,544 B.

## CLI and packaged-consumer exercise

Installed the packaged crate only into a new temporary consumer with:

```sh
cargo install --path target/package/animation-shot-runner-0.1.0 --root <consumer>/install
```

The installed `shot-runner --help` exposed one binary, four useful subcommands, `--json`, and version help. From the consumer project directory, a five-shot manifest using the normal relative `shots.json` form produced this successful sequence:

- `plan --json` showed all five named shots, including boundary `fps: 0.001`.
- `run shots.json --yes --allow-command cp --json` rendered 5/5; each shot received a copied frame, contact sheet, and receipt.
- A second identical run returned `rendered: 0, cache_hits: 5`.
- All five receipts verified. After deliberately replacing one output frame, `verify` failed with exit 5 and `hash mismatch`; re-running only that shot restored the cached frame and verification passed.
- Omitted `--yes` and a non-matching allowlist both returned exit 3 JSON errors. An unknown `--shot`, duplicate `init`, and a parent-directory output manifest returned exit 2 JSON errors.

The prior P0 is fixed: the normal relative-manifest run succeeds from the manifest directory, including its renderer working directory.

### Release-blocking reproduction: hidden command arguments

Create a valid manifest whose command is:

```json
["cp", "frame.png", "/tmp/this-destination-is-not-shown-by-plan.png"]
```

Then run `shot-runner plan hidden-args.json`:

```text
REVIEW  1 command(s)
  sq010                    cp  24 fps  sRGB
No commands executed.
```

The JSON plan likewise contains only `executable: "cp"`, source, FPS, and colorspace. It omits both arguments and their expanded values. `run hidden-args.json --yes --allow-command cp` would execute the full hidden argument vector. Exact executable allowlisting does not make arbitrary renderer flags/paths safe (for example, renderer script/plugin flags); the sole review UI expressly recommended by the README is insufficient.

## Site, PWA, privacy, and deployment verification

- Compared SHA-256 values for local production output and live responses. They match for `index.html`, `sw.js`, both hashed assets, both self-hosted fonts, both proof images, and `/privacy/` and `/terms/`. The live PWA therefore contains the candidate's `shot-runner-v2` worker. The remote URL can establish the static docs identity, not the separately packaged local binary identity.
- Desktop (1440 px) and phone (390 px) tests loaded without console/page errors. At 390 px there was no horizontal overflow. Both planned the five-shot sample, accepted `fps: 0.001`, and gave clear malformed-JSON recovery guidance.
- Keyboard-only traversal reached controls with visible 3 px focus outlines. Reduced-motion browser context computed `0s` animation and transition duration. Axe reported no serious/critical findings.
- No outbound request occurred on initial load. The manifest demo stayed in-browser. Source inspection found no CLI network code, telemetry, analytics, CDN font/script, `eval`, or media upload. The optional license flow is limited to the documented Sociobot endpoint, stores its token/verdict locally, and is not a core-renderer gate.
- On both local production output and live URL, the worker controlled the page after reload; `registration.update()` resolved with an active worker. A 390 px offline reload displayed the cached page and offline notice with no errors. Cache Storage had 11 public documentation entries and no license/token/entitlement/`/verify` URLs, including after a routed `?license=qa-token` verification flow.
- `/privacy/` and `/terms/` returned 200. Live responses have HSTS, strict-origin referrer policy, and `nosniff`.

## Defects

### P1 — `plan` does not expose what `run` confirms and executes

**Impact:** The contract is a local tool that does not execute untrusted manifests without confirmation. The operator is told to inspect `plan` then pass `--yes`; however `plan` conceals all command arguments. Allowlisting an executable name is not enough to safely approve its arbitrary arguments. This can turn the required confirmation into a blind confirmation.

**Required fix:** Show the complete, tokenized command vector in both human and JSON plan output (ideally also display the exact expansion with a clear source/cache context), then add an installed-binary integration test proving all arguments are visible before `--yes`. Alternatively require an explicit digest of the reviewed command vector at run time. Do not log sensitive material if future placeholders can contain it.

### P2 — live cache policy does not apply the shipped immutable-asset rules

`site/public/_headers` declares one-year immutable caching for `/assets/*`, fonts, and proof images, but the live URL returns `cache-control: public, must-revalidate, max-age=30` for HTML, `assets/main-DNe3Gmpc.js`, fonts, proof images, and `sw.js`. The PWA functions offline, but deployment cache policy does not meet the stated long-lived hashed/static-asset policy.

**Required fix:** configure the deployment host to honor the repository header rules (keep `sw.js` no-cache; give immutable hashes/assets long TTL) and verify live response headers after redeploy.

### P3 — deployment hardening headers are incomplete

Live responses lack Content-Security-Policy, frame-ancestors/X-Frame-Options, Permissions-Policy, and COOP. This did not create a functional or axe failure, but a conservative CSP and framing policy are appropriate for a page that accepts a license token in its URL.

## Required next steps

1. Fix the P1 command-review/confirmation boundary and add regression coverage that fails if any command argument is omitted from `plan`.
2. Apply and verify the P2 production cache policy.
3. Add the P3 response headers where compatible with the Sociobot purchase flow.
4. Cut a new commit, rerun the clean package/consumer verification, and deploy it before requesting another release verdict.
