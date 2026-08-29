# Handoff — adversarial review 3

Completed a read-only product review against commit `224935b96570655b27fbf4e26d39dedaaad87cc8` and the live site. Product code was not changed. The full result is in `.factory/review-3.md`.

## Verdict

**FAIL.** Six findings remain: a blocking unusable registry install command, a blocking incomplete claims inventory, first-demo text at 1.10:1 contrast, phone targets below 44 × 44 px, retained demo state after **Start for real**, and inconsistent approval terminology.

## Verification performed

- Fresh Chromium contexts at 390 × 844 and 1440 × 900 for cold home and demo reads.
- Live request/storage logging, Reset demo, Start for real, deep links, browser Back, 404, metadata, header/footer, and link crawl.
- Every command in `.factory/claims.json`, independently, after `npm ci` in `/tmp/shot-runner-review-3.pDIZPy/repo`.
- CLI demo from a separate temporary caller folder with a sentinel file.
- `npm test`, `npm run build`, `npm run pack:cli`, and live `npm run test:a11y`.
- SHA-256 comparison of live pages/assets with the clean candidate build.
- Direct probe of the exact displayed registry install command, which failed because the package is not on crates.io.

## Next steps

Resolve F-3-1 through F-3-6 in severity order, extend claim and rendered-accessibility coverage, deploy, and rerun the entire review from a fresh context. Do not treat the passing axe or listed claim suite as coverage for the manually measured contrast, target-size, or unlisted-claim defects.
