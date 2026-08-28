# Handoff — polish round 1

Shot Runner now has a runnable five-shot local demo, an isolated browser demo route, direct first-screen copy, complete route metadata, a designed 404, and a claim registry with runnable evidence.

## Run

```sh
npm ci
npm test
npm run build
npm run test:a11y
npm run pack:cli
```

Use `cargo run -p animation-shot-runner -- demo` to run the bundled Paper Courier sample. It prints its new temporary output folder. Website demo: `/demo/?demo=1`.

## Evidence

- `npm test`: Rust unit/integration tests, every claim test, static site checks, and PWA offline regression pass.
- `npm run test:a11y`: 39 axe passes, 0 violations, 0 serious/critical.
- `npm run build`: release CLI and `dist/site/` build pass; initial JS is 0.88 KB gzip and CSS is 3.67 KB gzip.
- `npm run pack:cli`: package and package verification pass.
- Screenshots: `.factory/evidence/home-390.png`, `home-1440.png`, and `demo-390.png`; capture reports no console errors and all tested 390 px routes are 390 px wide.
- Claim commands are listed in `.factory/claims.json`; `npm run test:claims` runs all of them from clean state.

## Known gaps

None in the reviewed scope. The factory deployment process still owns production publication and DNS.
