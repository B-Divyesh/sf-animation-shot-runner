# Demo contract

## Entry points

- Website: `/demo/?demo=1` opens the visible sample page. Visiting `/?demo=1` redirects to that real route.
- CLI: `shot-runner demo` runs the same bundled Paper Courier sample.

## Sample and isolation

`examples/paper-courier/` contains five named scene notes and a five-shot manifest. `shot-runner demo` copies that directory to a newly named system temporary folder, replaces its sample-only renderer command with the current binary’s harmless `demo-renderer`, and invokes the normal render, cache, contact-sheet, receipt, and receipt-verification pipeline. It prints the output folder. It never reads or writes a user project folder.

The browser page stores only `demo:animation-shot-runner:opened`. The persistent banner says “Demo — sample data, nothing is saved” and provides **Reset demo** and **Start for real**. Reset recreates only that demo key. Start for real removes every `demo:animation-shot-runner:*` key, preserves non-demo storage, and returns to `/`.

## Verification

Every claim in `claims.json` is tested from a fresh process or browser context through `npm run test:claims`. The browser test begins at `/?demo=1`; the CLI tests use only the bundled sample and remove their generated temporary folders.
