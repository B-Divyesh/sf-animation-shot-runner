use animation_shot_runner::{RunnerError, plan, run, starter_manifest, verify};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::{fs, path::PathBuf, process::ExitCode};

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
    /// Validate and show commands without executing them
    Plan {
        #[arg(help = "Manifest to validate")]
        manifest: PathBuf,
        #[arg(long, help = "Plan only this named shot")]
        shot: Option<String>,
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
        Commands::Plan { manifest, shot } => {
            let items = plan(manifest, shot.as_deref())?;
            print_value(&items, cli.json, |items| {
                println!("REVIEW  {} command(s)", items.len());
                for i in items {
                    println!(
                        "  {:<24} {}  {} fps  {}",
                        i.name, i.executable, i.fps, i.colorspace
                    );
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
    }
    Ok(())
}
