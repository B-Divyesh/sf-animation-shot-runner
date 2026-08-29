# Handoff — polish round 2

Repaired the review-2 release candidate. The distinct warm-newsprint broadsheet identity remains intact.

## Delivered

- The one-click `/demo/?demo=1` path now shows the real `shot-runner demo` transcript and a real generated contact sheet in its first mobile viewport. It retains the isolated banner, Reset demo, and Start for real controls.
- Added 12 behavior-level claims in `.factory/claims.json`, including cache/tamper, caller-folder isolation, exact planned argv, ordinary-run outputs/cache, renderer dependency boundaries, direct argv expansion, documented relative paths/exits, browser isolation, and MIT license coverage.
- Added complete Twitter metadata to Demo, Privacy, Terms, and 404; tightened receipt and exit-code wording; refreshed screenshot evidence and copy audit.

## Verification

- `npm ci`
- Every exact command in `.factory/claims.json` from the repaired clean tree.
- `npm test` — Rust unit/integration, site contract, all claims, and PWA offline regression passed.
- `npm run build` — `dist/site/` produced; initial JS 0.88 kB gzip and CSS 3.84 kB gzip.
- `npm run test:a11y` — 39 axe passes, 0 violations, 0 serious/critical.
- `npm run pack:cli` — publishable crate package passed.
- `node site/tests/capture.mjs` — fresh 390 px demo/home and 1440 px home screenshots with no console errors.

Local evidence: `.factory/evidence/demo-390.png`, `.factory/evidence/home-390.png`, and `.factory/evidence/home-1440.png`. A fresh clone at `/tmp/shot-runner-polish-2.Ye2g2G` passed the same suite and produced `target/package/animation-shot-runner-0.1.0.crate`. Repair commit: `a4430684b4d7e7427a866d9261d9fa104e26353c`.

Deployed through `/opt/fleet/lib/deploy-static.sh animation-shot-runner dist/site` (Azure deployment `188be8cc-77fc-4157-9870-28e11258a6c4`). A cold live 390 × 844 Chromium check at `https://animation-shot-runner.sociobot.in/demo/?demo=1` found title `Demo — Shot Runner`, one h1 `See five sample previews run.`, the demo banner, first-viewport contact-sheet image, no console errors, and no axe serious/critical issues. Live `/`, `/demo/`, `/privacy/`, and `/terms/` returned their pages; an unknown path returned HTTP 404. Screenshot: `.factory/evidence/live-demo-390.png`.

## Run and deploy

Use `cargo run -p animation-shot-runner -- demo` for the isolated CLI sample. Use `npm run build` to create `dist/site/`. Deployment remains the factory static work order; pushing `main` is the configured handoff trigger.

## Known gaps

None known.
