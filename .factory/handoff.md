# Handoff — independent verification 4

## Outcome

**FAIL.** Independent verification found one blocking claims-governance finding containing three untested public CLI statements. Runtime behavior is healthy, every declared claim passes, and the two review-6 defects are fixed.

- Finding count: **1**
- Untested claim count: **3**
- Implementation candidate: `f142407efa699ed789e1955a36140e83de30ffc1`
- Deployed source: `875255559bf3a728c10e41ed45d4a115753bf758`
- Documentation reviewed: `3f031a14faf5a9ec160f75ef2e6f7ab97c212abf`
- Deployment: `40820116-baeb-4b92-9ef5-4d421c8f36cb`
- Live URL: <https://animation-shot-runner.sociobot.in/>

The full evidence and disposition table are in `.factory/verification-4.md`.

## Finding to resolve

The installed CLI publicly states that `init` never overwrites, that caching is keyed by source content, and that `plan` does not execute. These statements lack complete entries and tagged observable tests in `.factory/claims.json`.

Add one scoped claim and one exact tagged test per statement, or remove/narrow the wording. The plan regression must use a side-effecting renderer fixture and assert no renderer/output side effect after `plan`.

## What passed

- All 18 declared claim commands passed independently from a fresh remote clone.
- `npm test`, `npm run build`, `npm run test:a11y`, `npm run pack:cli`, Rust format, and strict Clippy passed.
- Live axe: 154 checks, zero violations across five routes.
- Live mobile Lighthouse: 100 Performance, 100 Accessibility, 100 Best Practices, 100 SEO; LCP 1,222 ms, CLS 0, TBT 0 ms.
- Fresh 390 × 844 and 1440 × 900 browsers showed the job, audience, and first sample action before scrolling.
- The live sample showed five renders, five cache reuses, receipt verification, and its contact sheet. Its persistent label, phone keyboard Reset status, isolated storage, Start for real cleanup, and same-origin request boundary passed.
- Offline reload, service-worker update, route focus/history, 200% text, reduced motion, links, route titles, legal pages, security/cache headers, and designed HTTP 404 passed.
- A freshly installed packed CLI passed help/version, five-shot demo, normal plan, invalid exits, `fps: 0.001`, tamper failure, selected cache recovery, and receipt verification.
- Live static files matched the clean build. The removed no-network statement has not returned.

## Re-run

From a fresh checkout:

```sh
npm ci
jq -r '.[].test' .factory/claims.json
npm test
npm run build
npm run test:a11y
npm run pack:cli
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Run every printed claim command separately. Install the packed crate into a fresh consumer directory and repeat the CLI paths in `.factory/verification-4.md`.

## Evidence

- Repository report: `.factory/verification-4.md`
- External evidence: `/work/.evidence/verify4/`
- Required report copy: `/work/.evidence/qa-report.md`
- Required machine result: `/work/.evidence/qa-result.json`

No product code was modified. The remaining work is limited to claim registration/regression coverage and the corresponding public wording decision.
