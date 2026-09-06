# Handoff — independent review 6

## Outcome

Review 6 completed with **FAIL**. No product code was changed. The report is `.factory/review-6.md`.

- Implementation candidate: `29f2312feb8ae2e66637149b9b569dbf9d4b9c86`
- Documentation reviewed: `4b5fa6d2ceb1c3612b3b4474caeaf7d97956dfe2`
- Findings: 2
- Untested claims: 1

## Findings to resolve

1. `shot-runner --help` says **“No shell and no network are used.”** The no-network half is absent from `.factory/claims.json` and is broader than the tested behavior.
2. **Reset demo** changes its marker, but the completion message is hidden at 390 px and has no live-region semantics at any size.

## Verification completed

- Opened the live Home and Demo in fresh 390 × 844 and 1440 × 900 Chromium contexts.
- Ran every exact claim command independently from a fresh checkout.
- Ran `npm test`, `npm run build`, `npm run test:a11y`, `npm run pack:cli`, formatting, and clippy from that checkout.
- Installed the packed crate into a fresh consumer directory and exercised help, version, init, plan, demo, run, verify, invalid exits, cache recovery, and caller-folder isolation.
- Checked live routes, deliberate 404, links, metadata, keyboard use, focus/history, reduced motion, axe, offline reload, request origin, storage cleanup, security/cache headers, build parity, and mobile Lighthouse.

All declared claims and quality commands passed. Lighthouse scored 99 Performance and 100 for Accessibility, Best Practices, and SEO. The review remains FAIL because acceptance requires zero findings and zero untested claims.

## Recheck after repair

Run the documented clean chain and every exact claim command. Add a 390 px keyboard test proving reset feedback is visible and exposed through a polite status region. Remove or accurately narrow and register the CLI no-network sentence.
