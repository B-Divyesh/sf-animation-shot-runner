# Shot Runner

Shot Runner is a local, manifest-driven preview renderer for small animation teams and technical artists. It runs the renderer commands you explicitly approve, caches frame sequences by source content, makes contact sheets, and writes a portable JSON receipt containing hashes, FPS, colorspace, command, and outputs. Media never leaves your machine.

## Install

Prebuilt binaries will be attached to releases. From source (Rust 1.85+):

```sh
cargo install --path crates/shot-runner
shot-runner --help
```

Shot Runner does not bundle renderers. Install and license Blender, Motion Canvas, ffmpeg, or any other command used by your manifest according to that tool's terms. ffmpeg is only required if your own renderer command uses it; native PNG/JPEG contact sheets need no ffmpeg.

## Usage

Create a starter manifest:

```sh
shot-runner init shots.json
```

A manifest names each shot and gives a tokenized command. Placeholders are expanded without a shell: `{source}`, `{frames}`, `{shot}`, and `{cache}`.

```json
{
  "version": 1,
  "project": "paper-courier",
  "output": "previews",
  "shots": [
    {
      "name": "sq010-door",
      "source": "scenes/door.blend",
      "fps": 24,
      "colorspace": "sRGB",
      "command": ["blender", "-b", "{source}", "-o", "{frames}/frame_", "-a"]
    }
  ]
}
```

Inspect first, then run with both trust gates:

```sh
shot-runner plan shots.json
shot-runner run shots.json --allow-command blender --yes
shot-runner run shots.json --shot sq010-door --allow-command blender --yes --json
shot-runner verify previews/sq010-door/receipt.json --json
```

`plan` never executes commands. `run` refuses executable names that are not explicitly allowlisted and refuses to run without `--yes`; this is deliberate protection against untrusted manifests and works predictably in CI. A successful run writes `contact-sheet.png`, a copied frame sequence, and `receipt.json` under the manifest's output directory. Re-running unchanged inputs uses the content-addressed local cache.

For `shot-runner run shots.json`, the manifest directory is the current directory: relative sources, output, cache, and renderer working directory all resolve there. This is also true when the manifest is in a named relative directory such as `project/shots.json`.

Exit codes: `0` success, `2` manifest/usage error, `3` trust denied, `4` renderer failed, `5` output or verification failed.

## Develop and verify

```sh
npm install
npm test
npm run build
npm run pack:cli
```

`npm test` runs Rust tests and website checks. `npm run build` creates the deployable documentation site at `dist/site/`. `cargo package --manifest-path crates/shot-runner/Cargo.toml` creates the publishable Rust crate; the factory owns publication credentials.

## Privacy and licensing

Shot Runner has no telemetry and no network code. Sources, frames, caches, and receipts stay local. The optional Producer Toolkit license is restored and verified by the documentation site through Sociobot; it does not gate the core renderer, accessibility, safety, or receipt export. See [`site/privacy/index.html`](site/privacy/index.html) and [`site/terms/index.html`](site/terms/index.html).

MIT licensed. See [LICENSE](LICENSE) and [CHANGELOG.md](CHANGELOG.md).
