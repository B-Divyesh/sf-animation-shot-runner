# Changelog

## Unreleased

- Made `plan` disclose both every manifest command token and the exact expanded argv that `run` uses; receipts now provide a direct review/execution parity record.
- Made manifest-relative command expansions absolute and added coverage for both bare and named relative manifest paths.
- Added Azure Static Web Apps policy configuration for immutable content-named assets, a compatible CSP, anti-framing, Permissions-Policy, and COOP headers.
- Made the accessibility command self-contained by building and serving the production site before scanning it.
- Fixed `shot-runner run shots.json` so a bare relative manifest uses the current directory as its renderer working directory.
- Restricted the documentation service worker to public same-origin static assets; license-bearing URLs and Sociobot entitlement verification requests and responses never enter Cache Storage.
- Added executable CLI and browser Cache Storage regressions, and made the accessibility scan create its evidence directory in a clean checkout.

## 0.1.0 — 2026-08-27

- Initial Shot Runner CLI with manifest planning, explicit command trust gates, content-addressed caches, contact sheets, receipts, and verification.
- Initial static documentation, local manifest demo, and Producer Toolkit license restore flow.
