# Handoff — adversarial review 4 repair

## Outcome

The review-4 repair is deployed at <https://animation-shot-runner.sociobot.in/>.

- Release code commit: `fd536ab3106e1b314cd4b6735332a2d7d1f526d1`
- Deployment ID: `207d0ad5-7859-4da2-a2e1-e5dbf61c02b7`
- Verdict: **PASS.** F-4-1/F-2-12, F-4-2, and F-4-3 are fixed. No prior finding regressed.

## What changed

- Defined a receipt at the first Home and Demo uses; elsewhere used the clear term “JSON output records.”
- Replaced cryptic masthead/edition labels with **Shot Runner** and a useful local/MIT fact.
- Added real document-route scroll/focus restoration and polite route announcements for new navigation, Back, and Forward.
- Added the `route-history` claim/test, documented its short-lived per-tab route marker, and retained the isolated demo namespace.
- Tightened the phone header and demo spacing so the full wordmark and real contact-sheet output remain visible without horizontal overflow.
- Updated the copy audit and verb-first catalog description.

## Verification

Fresh remote clone `/tmp/shot-runner-polish4-clean.GFL9Sy/repo` at the release code commit:

- `npm ci` passed.
- Every exact command in all 18 `.factory/claims.json` entries passed independently.
- `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `npm test`, `npm run build`, `npm run test:a11y`, and `npm run pack:cli` passed.
- Local Lighthouse mobile: 100 Performance, 100 Accessibility, 100 Best Practices, 100 SEO; LCP 1,359 ms, CLS 0, TBT 0. Report: `.factory/evidence/lighthouse.json`.

Post-deploy cold checks:

- `/opt/fleet/lib/verify-url.sh` returned 200 with zero console errors, title/lang/one h1/main/alt/button checks passing. Evidence: `.factory/evidence/live-polish4/verify.json`.
- Live `npm run test:browser` passed at 390 × 844; live axe reported 154 checks and 0 violations across Home, Demo, Privacy, Terms, and 404.
- Live `@claim:isolated-browser-demo`, `@claim:route-history`, and `@claim:offline-opened-pages` passed.
- Live `/`, `/demo/?demo=1`, `/privacy/`, and `/terms/` returned 200; `/missing-polish-4` returned the designed 404.
- Built/live SHA-256 values match for Home, Demo, Privacy, Terms, 404, and `sw.js`.

Screenshots: `.factory/evidence/live-polish4-home-390.png`, `.factory/evidence/live-polish4-demo-390.png`, and `.factory/evidence/live-polish4-home-1440.png`.

## Run and release

- Develop/test: `npm ci && npm test`
- Build CLI and static site: `npm run build`
- Package without publishing: `npm run pack:cli`
- The CLI sample: `cargo run -p animation-shot-runner -- demo`

## Known gaps

None. The product remains a local CLI plus static documentation site; no publishing was attempted.
