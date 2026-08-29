# Shot Runner

Render named animation previews from one command.

Shot Runner is for small animation teams and technical artists. It runs renderer commands you approve. It writes frames, contact sheets, and JSON receipts on your computer. A receipt records output hashes, frame rate, and colour space.

## Try the bundled sample

Run the five-shot Paper Courier sample after installing from GitHub:

```sh
cargo run -p animation-shot-runner -- demo
```

The command copies bundled sample files into a new system temporary folder. It renders five named shots. It repeats the run to show cached frames. It checks a receipt and prints the output folder. Your project files are not used.

Open [`/demo/?demo=1`](https://animation-shot-runner.sociobot.in/demo/?demo=1) for the same sample output. The browser demo uses separate `demo:` storage. **Reset demo** recreates its marker. **Start for real** discards every demo key and returns home.

## Install

Install the pinned source revision from GitHub:

```sh
cargo install --git https://github.com/B-Divyesh/sf-animation-shot-runner.git --rev 224935b96570655b27fbf4e26d39dedaaad87cc8 --locked animation-shot-runner
shot-runner --help
```

The first command is the tested install path for this release. See the [source and install notes](https://github.com/B-Divyesh/sf-animation-shot-runner#install).

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

`plan` does not execute commands. `run` requires an approved executable name and `--yes`. A successful run writes copied frames, `contact-sheet.png`, and `receipt.json`. A second unchanged run uses the local cache.

With `shot-runner run project/shots.json`, paths resolve from the `project` folder. Use the same `--cache-dir` value with `plan` and `run` when you set one.

Exit 0 means success. Exit 2 means the shot file is invalid. Exit 3 means approval is missing. See `shot-runner --help` for other errors.

## Develop and verify

```sh
npm ci
npm test
npm run build
npm run test:a11y
npm run pack:cli
```

`npm run build` creates the release CLI and deployable site. The outputs are `target/release/shot-runner` and `dist/site/`.

`npm run pack:cli` creates a checked Rust package in `target/package/`. Do not publish the crate from this repository.

## Privacy and license

The CLI reads and writes local paths. See [Privacy](site/privacy/index.html) and [Terms](site/terms/index.html).

MIT licensed. See [LICENSE](LICENSE) and [CHANGELOG.md](CHANGELOG.md).
