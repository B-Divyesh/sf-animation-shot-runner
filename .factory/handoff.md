# Handoff — adversarial review 4

## Outcome

Review 4 is complete at candidate `ad7a95c348c00e0ccdbcd480927c16f7e3008952` and live URL <https://animation-shot-runner.sociobot.in/>.

Verdict: **FAIL**. The full report is `.factory/review-4.md`. No product code was changed.

## Findings left for the next repair round

- **F-4-1 / F-2-12 — BLOCKING:** Home and Demo use **receipt** before explaining that it is a JSON record of output hashes, frame rate, and colour space. This is a half-fixed earlier finding.
- **F-4-2 — HIGH:** Home → Demo → Back reaches the correct URL but loses y=1200 scroll state; focus remains on `<body>` during forward, back, and forward-again navigation.
- **F-4-3 — MINOR:** **SR / 01** and **VOL. 01 — 2026** are decorative/cryptic labels rather than useful plain words.

## Verification performed

- Fresh Chromium cold reads at 390 × 844 and 1440 × 900.
- One-click live demo with request logging, real/demo storage sentinels, Reset, Start for real, transcript/contact-sheet first-screen checks, and console capture.
- Manual live deep-link, 404, metadata, header/footer, fragment, external-link, Back/Forward, scroll, and focus checks.
- Live browser-quality suite, five-route axe suite, and `/opt/fleet/lib/verify-url.sh`.
- Every one of the 17 exact `.factory/claims.json` commands, independently, after `npm ci` in clean clone `/tmp/shot-runner-review4-clean.ekot2m/repo`.
- A separate `shot-runner demo` run from `/tmp/shot-runner-review4-caller.*`, confirming five renders, five cache hits, five receipts, five contact sheets, and caller-file isolation.
- Clean-clone `npm test`, `npm run build`, `npm run test:a11y`, and `npm run pack:cli`.
- Built/live SHA-256 comparison for Home, Demo, Privacy, Terms, 404, and `sw.js`; all matched.

## Passing evidence summary

- All 17 declared claims passed; no listed claim is untested.
- Demo sandbox, offline reload, same-origin request boundary, Reset, and exit cleanup passed.
- Clean build produced the release CLI and `dist/site/`.
- Axe reported 155 checks and zero violations across five routes.
- Live route/link/404/metadata/security-header checks passed except for the scroll/focus behavior in F-4-2.
- The distinct broadsheet identity matches `.factory/design.md`.

## Next steps

Repair only the three findings above, add the specified terminology and navigation regressions, redeploy, and rerun the entire adversarial checklist from a fresh browser and clean clone.
