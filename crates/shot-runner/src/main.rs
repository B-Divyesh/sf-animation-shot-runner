use animation_shot_runner::{RunnerError, plan, run, starter_manifest, verify};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(
    name = "shot-runner",
    version,
    about = "Repeatable local preview renders, contact sheets, and receipts",
    long_about = "Shot Runner executes tokenized renderer commands from a JSON manifest only after explicit confirmation and allowlisting. It stores frames locally, caches by source content, and writes verifiable receipts. No shell and no network are used."
)]
struct Cli {
    #[arg(long, global = true, help = "Print machine-readable JSON")]
    json: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Write a documented starter manifest (never overwrites)
    Init {
        #[arg(default_value = "shots.json", help = "Path for the new JSON manifest")]
        path: PathBuf,
    },
    /// Validate and show every token plus the exact expanded argv without executing
    Plan {
        #[arg(help = "Manifest to validate")]
        manifest: PathBuf,
        #[arg(long, help = "Plan only this named shot")]
        shot: Option<String>,
        #[arg(
            long,
            value_name = "DIRECTORY",
            help = "Use this cache directory when expanding argv; pass the same value to run"
        )]
        cache_dir: Option<PathBuf>,
    },
    /// Execute reviewed commands and write previews plus receipts
    Run {
        #[arg(help = "Reviewed manifest to execute")]
        manifest: PathBuf,
        #[arg(long, help = "Run only this named shot")]
        shot: Option<String>,
        #[arg(long = "allow-command", value_name = "EXECUTABLE", action = clap::ArgAction::Append, help = "Allow one exact executable token; repeat for multiple tools")]
        allow_command: Vec<String>,
        #[arg(
            long,
            help = "Confirm that every command in the plan has been reviewed"
        )]
        yes: bool,
        #[arg(
            long,
            value_name = "DIRECTORY",
            help = "Override the local content cache directory"
        )]
        cache_dir: Option<PathBuf>,
    },
    /// Verify every output hash recorded by a receipt
    Verify {
        #[arg(help = "Receipt JSON to verify")]
        receipt: PathBuf,
    },
    /// Run five bundled sample shots in a new temporary folder
    Demo,
    #[command(hide = true)]
    /// Internal harmless renderer used only by the bundled demo
    DemoRenderer {
        #[arg(help = "Bundled sample source file")]
        source: PathBuf,
        #[arg(help = "PNG frame to write")]
        destination: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match execute(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if cli.json {
                println!(
                    "{}",
                    serde_json::json!({"ok": false, "code": error.code, "error": error.message})
                );
            } else {
                eprintln!("shot-runner: {}", error.message);
            }
            ExitCode::from(error.code as u8)
        }
    }
}

fn print_value<T: Serialize>(
    value: &T,
    json: bool,
    human: impl FnOnce(&T),
) -> Result<(), RunnerError> {
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(value).map_err(|e| RunnerError {
                code: 5,
                message: e.to_string()
            })?
        );
    } else {
        human(value);
    }
    Ok(())
}

fn execute(cli: &Cli) -> Result<(), RunnerError> {
    match &cli.command {
        Commands::Init { path } => {
            if path.exists() {
                return Err(RunnerError {
                    code: 2,
                    message: format!("{} already exists; refusing to overwrite", path.display()),
                });
            }
            fs::write(path, starter_manifest()).map_err(|e| RunnerError {
                code: 5,
                message: format!("could not write {}: {e}", path.display()),
            })?;
            if cli.json {
                println!("{}", serde_json::json!({"ok": true, "manifest": path}));
            } else {
                println!(
                    "Wrote {}\nNext: shot-runner plan {}",
                    path.display(),
                    path.display()
                );
            }
        }
        Commands::Plan {
            manifest,
            shot,
            cache_dir,
        } => {
            let items = plan(manifest, shot.as_deref(), cache_dir.as_deref())?;
            print_value(&items, cli.json, |items| {
                println!("REVIEW  {} command(s)", items.len());
                for i in items {
                    println!(
                        "  {:<24} {}  {} fps  {}",
                        i.name, i.executable, i.fps, i.colorspace
                    );
                    println!(
                        "    manifest argv: {}",
                        serde_json::to_string(&i.command).expect("plan command serializes")
                    );
                    println!(
                        "    run argv:      {}",
                        serde_json::to_string(&i.argv).expect("plan argv serializes")
                    );
                    println!("    frames:        {}", i.frames_directory);
                }
                println!("No commands executed.");
            })?;
        }
        Commands::Run {
            manifest,
            shot,
            allow_command,
            yes,
            cache_dir,
        } => {
            let result = run(
                manifest,
                shot.as_deref(),
                allow_command,
                *yes,
                cache_dir.as_deref(),
            )?;
            print_value(&result, cli.json, |s| {
                println!(
                    "DONE  {} — {} rendered, {} from cache",
                    s.project, s.rendered, s.cache_hits
                );
                for r in &s.receipts {
                    println!("  {r}");
                }
            })?;
        }
        Commands::Verify { receipt } => {
            let result = verify(receipt)?;
            if !result.valid {
                return Err(RunnerError {
                    code: 5,
                    message: format!(
                        "receipt verification failed: {}",
                        result.mismatches.join("; ")
                    ),
                });
            }
            print_value(&result, cli.json, |v| {
                println!("VERIFIED  {} output file(s) match the receipt", v.checked);
            })?;
        }
        Commands::Demo => run_demo(cli.json)?,
        Commands::DemoRenderer {
            source,
            destination,
        } => render_demo_frame(source, destination)?,
    }
    Ok(())
}

fn run_demo(json: bool) -> Result<(), RunnerError> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let directory =
        env::temp_dir().join(format!("shot-runner-demo-{}-{nonce}", std::process::id()));
    seed_demo_samples(&directory)?;

    let binary = env::current_exe().map_err(|e| RunnerError {
        code: 5,
        message: format!("could not locate shot-runner: {e}"),
    })?;
    let manifest_path = directory.join("shots.json");
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&manifest_path).map_err(|e| RunnerError {
            code: 5,
            message: format!("could not read bundled manifest: {e}"),
        })?)
        .map_err(|e| RunnerError {
            code: 5,
            message: format!("could not parse bundled manifest: {e}"),
        })?;
    for shot in manifest["shots"]
        .as_array_mut()
        .ok_or_else(|| RunnerError {
            code: 5,
            message: "bundled manifest has no shots".into(),
        })?
    {
        shot["command"] = serde_json::json!([
            binary.to_string_lossy(),
            "demo-renderer",
            "{source}",
            "{frames}/frame-0001.png"
        ]);
    }
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).map_err(|e| RunnerError {
            code: 5,
            message: e.to_string(),
        })?,
    )
    .map_err(|e| RunnerError {
        code: 5,
        message: format!("could not prepare bundled manifest: {e}"),
    })?;

    let allowed = vec![binary.to_string_lossy().to_string()];
    let first = run(&manifest_path, None, &allowed, true, None)?;
    let second = run(&manifest_path, None, &allowed, true, None)?;
    let receipt = directory.join("previews/sq050-exit/receipt.json");
    let verified = verify(&receipt)?;
    if !verified.valid {
        return Err(RunnerError {
            code: 5,
            message: "bundled demo receipt did not verify".into(),
        });
    }
    if json {
        println!(
            "{}",
            serde_json::json!({"ok": true, "directory": directory, "rendered": first.rendered, "cache_hits_on_repeat": second.cache_hits, "receipt": receipt})
        );
    } else {
        println!("DEMO COMPLETE — {} sample shots rendered", first.rendered);
        println!(
            "CACHE CHECK — {} sample shots reused on repeat",
            second.cache_hits
        );
        println!(
            "RECEIPT VERIFIED — {} output files checked",
            verified.checked
        );
        println!("Output: {}", directory.display());
        println!("This folder contains only bundled sample data. Delete it when you are done.");
    }
    Ok(())
}

fn seed_demo_samples(destination: &Path) -> Result<(), RunnerError> {
    fs::create_dir_all(destination).map_err(|e| RunnerError {
        code: 5,
        message: format!("could not create demo folder: {e}"),
    })?;
    fs::create_dir_all(destination.join("sources")).map_err(|e| RunnerError {
        code: 5,
        message: format!("could not create demo sources: {e}"),
    })?;
    let files = [
        ("shots.json", include_str!("../demo_assets/shots.json")),
        (
            "sources/arrival.scene",
            include_str!("../demo_assets/arrival.scene"),
        ),
        (
            "sources/door.scene",
            include_str!("../demo_assets/door.scene"),
        ),
        (
            "sources/crossing.scene",
            include_str!("../demo_assets/crossing.scene"),
        ),
        (
            "sources/turn.scene",
            include_str!("../demo_assets/turn.scene"),
        ),
        (
            "sources/exit.scene",
            include_str!("../demo_assets/exit.scene"),
        ),
    ];
    for (relative, contents) in files {
        fs::write(destination.join(relative), contents).map_err(|e| RunnerError {
            code: 5,
            message: format!("could not write bundled sample: {e}"),
        })?;
    }
    Ok(())
}

fn render_demo_frame(source: &Path, destination: &Path) -> Result<(), RunnerError> {
    use image::{DynamicImage, ImageBuffer, Rgba};
    let bytes = fs::read(source).map_err(|e| RunnerError {
        code: 5,
        message: format!("could not read demo source: {e}"),
    })?;
    let seed = bytes
        .iter()
        .fold(0u8, |sum, value| sum.wrapping_add(*value));
    let image = ImageBuffer::from_fn(160, 90, |x, y| {
        let stripe = ((x / 16 + y / 18) % 2) as u8;
        Rgba([
            25 + seed / 8 + stripe * 20,
            35 + (x as u8 / 8),
            45 + (y as u8 / 4),
            255,
        ])
    });
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|e| RunnerError {
            code: 5,
            message: format!("could not create demo frames: {e}"),
        })?;
    }
    DynamicImage::ImageRgba8(image)
        .save(destination)
        .map_err(|e| RunnerError {
            code: 5,
            message: format!("could not write demo frame: {e}"),
        })
}
