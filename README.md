# Shot Runner

Render named animation previews from one command.

Shot Runner is for small animation teams and technical artists. It runs renderer commands you approve. It writes frames, contact sheets, and JSON receipts on your computer.

## Try the bundled sample

Run the five-shot Paper Courier sample after installing from source:

```sh
cargo run -p animation-shot-runner -- demo
```

The command copies bundled sample files into a new system temporary folder. It renders five named shots. It repeats the run to show cached frames. It checks a receipt and prints the output folder. Your project files are not used.

Open `/demo/?demo=1` on the documentation site for the same sample instructions. The browser demo marker uses a separate `demo:` local-storage key. Use **Reset demo** to reset it or **Start for real** to return home.

## Install

From source, using Rust 1.85 or newer:

```sh
cargo install --path crates/shot-runner
shot-runner --help
```

Shot Runner does not include a renderer. Install and license Blender, Motion Canvas, ffmpeg, or another renderer yourself. Native PNG and JPEG contact sheets do not need ffmpeg.

## Use your own shots

Create a shot file:

```sh
shot-runner init shots.json
```

A shot file lists a shot name, source path, frame rate, colour space, and command parts. The command parts are passed directly to the renderer. No shell is used. `{source}`, `{frames}`, `{shot}`, and `{cache}` are replaced before a command runs.

```json
{
  "version": 1,
  "project": "paper-courier",
  "output": "previews",
  "shots": [{
    "name": "sq010-door",
    "source": "scenes/door.blend",
    "fps": 24,
    "colorspace": "sRGB",
    "command": ["blender", "-b", "{source}", "-o", "{frames}/frame_", "-a"]
  }]
}
```

Review the exact command first:

```sh
shot-runner plan shots.json
shot-runner run shots.json --allow-command blender --yes
shot-runner verify previews/sq010-door/receipt.json
```

`plan` does not execute commands. `run` requires an approved executable name and `--yes`. Each successful shot writes copied frames, `contact-sheet.png`, and `receipt.json`. A second unchanged run uses the local cache.

With `shot-runner run project/shots.json`, paths resolve from the `project` folder. Use the same `--cache-dir` value with `plan` and `run` when you set one.

Exit codes are `0` for success, `2` for a shot-file error, `3` when approval is missing, `4` when the renderer fails, and `5` for output or receipt failures.

## Develop and verify

```sh
npm ci
npm test
npm run build
npm run test:a11y
npm run pack:cli
```

`npm run build` creates the deployable site in `dist/site/`. `cargo package --manifest-path crates/shot-runner/Cargo.toml` creates the publishable crate. Do not publish from this repository.

## Privacy and license

The CLI reads and writes local paths. See [Privacy](site/privacy/index.html) and [Terms](site/terms/index.html).

MIT licensed. See [LICENSE](LICENSE) and [CHANGELOG.md](CHANGELOG.md).
