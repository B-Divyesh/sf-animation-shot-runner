# Shot Runner — verification handoff

## Release verdict: **FAIL**

Independent QA of commit `8616729fa7190925bb42306f73dbd80f8aee5eb7` on 2026-08-27 found a P1 release blocker. The installed CLI's required review command (`plan`) hides command arguments, while `run --yes --allow-command <executable>` executes them. This makes the specified safe confirmation of an untrusted manifest blind. Do not release this candidate.

The previous relative-manifest and PWA sensitive-cache failures are fixed: the normal relative `shots.json` run works, and local/live Cache Storage contains no license, token, entitlement, or verification response.

Full evidence, exact reproducer, all passing gates, live asset hashes, and remaining P2/P3 deployment findings are in `.factory/verification-2.md`.

## How to run and verify

```sh
npm ci
npm test
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run build
npm run pack:cli
npm run test:a11y
```

`npm run build` writes static docs to `dist/site/`. `npm run pack:cli` creates the ready-to-publish Cargo crate at `target/package/animation-shot-runner-0.1.0/`; publication remains factory-owned.

## Required next work

1. Make `plan` reveal every exact command argument/expansion that `run` can execute, and regression-test that boundary.
2. Configure the live host to apply immutable caching to hashed/static assets; retain no-cache for the worker.
3. Add compatible CSP, anti-framing, Permissions-Policy, and COOP headers.
4. Rebuild, package/install in a clean consumer, deploy, and request a fresh verification.
