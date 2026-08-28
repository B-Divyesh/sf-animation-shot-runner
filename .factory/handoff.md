# Handoff — adversarial review 2

Completed a documentation-only adversarial review of the deployed Shot Runner candidate. Product source was not modified.

## Delivered

- Added `.factory/review-2.md` with cold 390 px and desktop checks, full landing/README copy audit with word counts, demo/sandbox evidence, claim-by-claim clean-clone results, route/metadata/link checks, history revalidation, and verdict.
- The verdict is **FAIL**. Blocking findings are the below-fold demo recording/first-screen result and incomplete claims coverage (including unlisted public claims). The report gives concrete fixes.

## Verification performed

- Fresh live Chromium contexts at 390 × 844 and 1440 × 900: no console errors, no horizontal overflow, same-origin requests only.
- Fresh browser demo: redirect, demo banner, separate `demo:` storage, Reset demo, Start for real, and request-log isolation checked.
- CLI `shot-runner demo` executed from a separate temporary working directory and printed five rendered shots, five cache hits, receipt verification, and an isolated output path.
- Fresh local clone `/tmp/shot-runner-review-2.WYtvD6`: all five exact commands in `.factory/claims.json` passed; `npm test`, `npm run build`, `npm run test:a11y` (39 passes, 0 violations), and `npm run pack:cli` passed.
- Live route/header checks confirmed normal routes/assets, sitemap/robots, and a designed HTTP 404.

## Known gaps

See blocking F-2-1 through F-2-10, plus F-2-11 and F-2-12, in `.factory/review-2.md`. No code changes were made in this review handoff.
