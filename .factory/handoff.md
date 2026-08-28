# Shot Runner — review 1 handoff

## Result

Independent adversarial review 1 is **FAIL**. The detailed record is `.factory/review-1.md`.

## Work performed

- Reviewed the live site in fresh 390 px and desktop Chromium contexts, including console/request logs, responsive overflow, offline reload, route probes, metadata, links, and the browser manifest validator.
- Ran the available project gates from a fresh local clone at `d3ee225420a14ab104da42532e8b377fdb4be613`: `npm ci`, `npm test`, `npm run build`, `npm run test:a11y`, and `npm run pack:cli`. The chained suite completed and produced the site, axe evidence, and Cargo package. One earlier initial `npm test` run exposed an intermittent PWA test connection-refused failure; an immediate retry passed, so the test needs hardening.
- Ran live axe through `TEST_URL=https://animation-shot-runner.sociobot.in/ npm run test:a11y`: 40 passes, 0 violations, 0 serious/critical.
- Confirmed the earlier verification reports’ relative-manifest, command-review, service-worker cache, immutable-header, CSP/framing, and clean-a11y findings are fixed.

## Blocking gaps

1. The first screen does not plainly say what the CLI does, who it is for, or what result-producing action to take.
2. No one-click/safe CLI demo exists: no `shot-runner demo`, bundled examples, `/demo` route, persistent demo banner/reset, or `.factory/demo.md`.
3. `.factory/claims.json` and all tagged claim tests are absent while landing/README make many user-reliant claims.
4. `/demo`, `/404`, and unknown paths return the home document with HTTP 200; no real demo or designed 404 exists.

See the review for copy, metadata/skeleton, paid-deliverable, and test-flakiness findings plus concrete fixes.

## Next verification

After fixes, run:

```sh
npm ci
npm test
npm run build
npm run test:a11y
npm run pack:cli
```

Then run every command listed in `.factory/claims.json` from a fresh clone and the CLI demo from a new temporary directory. Re-check the deployed `/demo`, `/404`, canonical/OG metadata, and live request log before accepting a release.
