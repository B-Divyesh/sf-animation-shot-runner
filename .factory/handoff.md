# Shot Runner — independent QA handoff

## Release verdict: PASS

Independent verification accepted candidate `e2d992be9b401f5ebdf848284ee9613e2e0259db` on 2026-08-27. The live documentation/PWA at <https://animation-shot-runner.sociobot.in/> matches the candidate's production static output. See `.factory/verification-3.md` for exact commands and evidence.

## What was independently verified

- Clean clone, `npm ci`, `npm test`, Rust format and clippy with warnings denied, and exact `npm run build` all passed.
- `npm run test:a11y` passed locally and live: 40 axe passes, zero violations, zero serious/critical findings.
- `npm run pack:cli` produced and Cargo-verified `animation-shot-runner v0.1.0`; it was installed only from that package into a clean consumer.
- The installed CLI planned, rendered, cached, receipt-verified, detected tampering, and recovered five named shots. It accepted `fps: 0.001`, rejected invalid FPS and trust violations with documented exit codes, and exposed complete reviewed command/argv vectors.
- Live desktop and 390 px mobile checks found no console/page errors or horizontal overflow; normal, invalid, and recovery manifest-demo paths worked; focus was visible; reduced motion was `0s`.
- The deployed PWA controlled its page, updated, reloaded offline, and did not cache license/token/entitlement data. Initial browser load made no outbound requests.
- Live headers, legal pages, CSP/framing protections, service-worker caching, self-hosted assets, and the static bundle budgets passed. SHA-256 values matched local built output for all served release assets checked.

## How to run and verify

```sh
npm ci
npm test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm run build
npm run test:a11y
npm run pack:cli
```

The deployable documentation site is `dist/site/`. Publish ownership remains with the factory; make the release artifact using `npm run pack:cli` and do not publish from this repository.

## Known gaps / next steps

No product defects or release blockers found. Lighthouse 13.4.1 could not produce a score because its Chrome tab crashed in this QA container; independent bundle, axe, responsive, offline, console, and browser checks passed. A future release verification should repeat the clean package-consumer and live asset-hash checks after deployment.
