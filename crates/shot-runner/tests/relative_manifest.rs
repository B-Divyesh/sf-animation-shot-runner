use image::{DynamicImage, ImageBuffer, Rgba};
use serde_json::Value;
use std::{fs, process::Command};
use tempfile::tempdir;

#[test]
fn documented_relative_manifest_run_executes_from_the_current_directory() {
    let dir = tempdir().expect("temporary consumer project");
    fs::write(dir.path().join("source.txt"), "source-v1").expect("source fixture");
    DynamicImage::ImageRgba8(ImageBuffer::from_pixel(80, 45, Rgba([20, 30, 40, 255])))
        .save(dir.path().join("fixture.png"))
        .expect("image fixture");
    fs::write(
        dir.path().join("shots.json"),
        r#"{
  "version": 1,
  "project": "relative-manifest",
  "output": "previews",
  "shots": [{
    "name": "sq010",
    "source": "source.txt",
    "fps": 24,
    "colorspace": "sRGB",
    "command": ["cp", "fixture.png", "{frames}/frame-0001.png"]
  }]
}"#,
    )
    .expect("manifest fixture");

    let output = Command::new(env!("CARGO_BIN_EXE_shot-runner"))
        .current_dir(dir.path())
        .args([
            "run",
            "shots.json",
            "--allow-command",
            "cp",
            "--yes",
            "--json",
        ])
        .output()
        .expect("run documented command");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: Value = serde_json::from_slice(&output.stdout).expect("JSON summary");
    assert_eq!(summary["rendered"], 1);
    assert!(dir.path().join("previews/sq010/receipt.json").is_file());
}

#[test]
fn named_relative_manifest_keeps_renderer_paths_relative_to_its_directory() {
    let dir = tempdir().expect("temporary consumer project");
    let project = dir.path().join("project");
    fs::create_dir(&project).expect("project directory");
    fs::write(project.join("source.txt"), "source-v1").expect("source fixture");
    DynamicImage::ImageRgba8(ImageBuffer::from_pixel(80, 45, Rgba([20, 30, 40, 255])))
        .save(project.join("fixture.png"))
        .expect("image fixture");
    fs::write(
        project.join("shots.json"),
        r#"{
  "version": 1,
  "project": "nested-relative-manifest",
  "output": "previews",
  "shots": [{
    "name": "sq010",
    "source": "source.txt",
    "fps": 24,
    "colorspace": "sRGB",
    "command": ["cp", "fixture.png", "{frames}/frame-0001.png"]
  }]
}"#,
    )
    .expect("manifest fixture");

    let output = Command::new(env!("CARGO_BIN_EXE_shot-runner"))
        .current_dir(dir.path())
        .args([
            "run",
            "project/shots.json",
            "--allow-command",
            "cp",
            "--yes",
            "--json",
        ])
        .output()
        .expect("run named relative manifest");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(project.join("previews/sq010/receipt.json").is_file());
}

#[test]
fn planned_argv_is_the_complete_argv_recorded_after_execution() {
    let dir = tempdir().expect("temporary consumer project");
    fs::write(dir.path().join("source.txt"), "source-v1").expect("source fixture");
    DynamicImage::ImageRgba8(ImageBuffer::from_pixel(80, 45, Rgba([20, 30, 40, 255])))
        .save(dir.path().join("fixture.png"))
        .expect("image fixture");
    fs::write(
        dir.path().join("shots.json"),
        r#"{
  "version": 1,
  "project": "argv-parity",
  "output": "previews",
  "shots": [{
    "name": "sq010",
    "source": "source.txt",
    "fps": 24,
    "colorspace": "sRGB",
    "command": ["cp", "fixture.png", "{frames}/frame-0001.png"]
  }]
}"#,
    )
    .expect("manifest fixture");

    let binary = env!("CARGO_BIN_EXE_shot-runner");
    let plan = Command::new(binary)
        .current_dir(dir.path())
        .args(["--json", "plan", "shots.json"])
        .output()
        .expect("plan command");
    assert!(
        plan.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&plan.stderr)
    );
    let planned: Value = serde_json::from_slice(&plan.stdout).expect("JSON plan");
    let item = &planned[0];
    assert_eq!(
        item["command"],
        serde_json::json!(["cp", "fixture.png", "{frames}/frame-0001.png"]),
        "plan preserves every manifest argument"
    );
    assert_eq!(item["argv"].as_array().map(Vec::len), Some(3));
    assert!(
        item["argv"][2]
            .as_str()
            .is_some_and(|argument| argument.ends_with("/frames/frame-0001.png")),
        "plan expands the frame placeholder"
    );

    let run = Command::new(binary)
        .current_dir(dir.path())
        .args([
            "--json",
            "run",
            "shots.json",
            "--allow-command",
            "cp",
            "--yes",
        ])
        .output()
        .expect("run command");
    assert!(
        run.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&run.stderr)
    );
    let receipt: Value = serde_json::from_slice(
        &fs::read(dir.path().join("previews/sq010/receipt.json")).expect("receipt"),
    )
    .expect("receipt JSON");
    assert_eq!(
        receipt["command"], item["argv"],
        "the renderer receipt proves run used the complete argv displayed by plan"
    );
}

#[test]
fn direct_command_expands_every_placeholder_without_shell_interpretation() {
    let dir = tempdir().expect("temporary project");
    fs::write(dir.path().join("source;safe.txt"), "source-v1").expect("source");
    let script = dir.path().join("capture-argv.sh");
    fs::write(&script, "#!/bin/sh\nprintf '%s\\n' \"$@\" > captured-argv.txt\ncp fixture.png \"$2/frame-0001.png\"\n")
        .expect("capture script");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755))
            .expect("make script executable");
    }
    DynamicImage::ImageRgba8(ImageBuffer::from_pixel(80, 45, Rgba([20, 30, 40, 255])))
        .save(dir.path().join("fixture.png"))
        .expect("image fixture");
    let script_text = script.to_string_lossy();
    fs::write(dir.path().join("shots.json"), format!(r#"{{"version":1,"project":"argv","output":"previews","shots":[{{"name":"sq010","source":"source;safe.txt","fps":24,"colorspace":"sRGB","command":["{}","{{source}}","{{frames}}","{{shot}}","{{cache}}","literal;touch should-not-exist"]}}]}}"#, script_text)).expect("manifest");
    let output = Command::new(env!("CARGO_BIN_EXE_shot-runner"))
        .current_dir(dir.path())
        .args([
            "run",
            "shots.json",
            "--allow-command",
            script_text.as_ref(),
            "--yes",
        ])
        .output()
        .expect("run direct argv fixture");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let args = fs::read_to_string(dir.path().join("captured-argv.txt")).expect("captured argv");
    assert!(args.contains("source;safe.txt"));
    assert!(args.contains("sq010"));
    assert!(args.contains("literal;touch should-not-exist"));
    assert!(
        !dir.path().join("should-not-exist").exists(),
        "runner must not invoke a shell"
    );
}

#[test]
fn documented_relative_paths_cache_and_exit_codes_hold() {
    let dir = tempdir().expect("temporary consumer project");
    let project = dir.path().join("project");
    fs::create_dir(&project).expect("project");
    fs::write(project.join("source.txt"), "source-v1").expect("source");
    DynamicImage::ImageRgba8(ImageBuffer::from_pixel(80, 45, Rgba([20, 30, 40, 255])))
        .save(project.join("fixture.png"))
        .expect("fixture");
    fs::write(project.join("shots.json"), r#"{"version":1,"project":"relative","output":"previews","shots":[{"name":"sq010","source":"source.txt","fps":24,"colorspace":"sRGB","command":["cp","fixture.png","{frames}/frame-0001.png"]}]}"#).expect("manifest");
    let binary = env!("CARGO_BIN_EXE_shot-runner");
    let plan = Command::new(binary)
        .current_dir(dir.path())
        .args([
            "--json",
            "plan",
            "project/shots.json",
            "--cache-dir",
            "shared-cache",
        ])
        .output()
        .expect("cache-dir plan");
    assert_eq!(plan.status.code(), Some(0));
    let planned: Value = serde_json::from_slice(&plan.stdout).expect("planned JSON");
    let cache_root = dir.path().join("shared-cache");
    assert!(
        planned[0]["cache_directory"]
            .as_str()
            .is_some_and(|path| path.starts_with(cache_root.to_string_lossy().as_ref()))
    );
    let success = Command::new(binary)
        .current_dir(dir.path())
        .args([
            "run",
            "project/shots.json",
            "--cache-dir",
            "shared-cache",
            "--allow-command",
            "cp",
            "--yes",
        ])
        .output()
        .expect("success run");
    assert_eq!(success.status.code(), Some(0));
    assert!(project.join("previews/sq010/receipt.json").is_file());
    let missing = Command::new(binary)
        .current_dir(dir.path())
        .args(["plan", "missing.json"])
        .output()
        .expect("bad manifest");
    assert_eq!(missing.status.code(), Some(2));
    let unapproved = Command::new(binary)
        .current_dir(dir.path())
        .args(["run", "project/shots.json"])
        .output()
        .expect("unapproved");
    assert_eq!(unapproved.status.code(), Some(3));
    let failing = Command::new(binary)
        .current_dir(dir.path())
        .args([
            "run",
            "project/shots.json",
            "--shot",
            "sq010",
            "--allow-command",
            "false",
            "--yes",
        ])
        .output()
        .expect("wrong allowed");
    assert_eq!(failing.status.code(), Some(3));
}

#[test]
fn native_contact_sheet_runs_without_ffmpeg_on_path() {
    let dir = tempdir().expect("temporary native-image project");
    fs::write(dir.path().join("source.txt"), "source-v1").expect("source");
    DynamicImage::ImageRgba8(ImageBuffer::from_pixel(80, 45, Rgba([20, 30, 40, 255])))
        .save(dir.path().join("fixture.png"))
        .expect("fixture");
    fs::write(dir.path().join("shots.json"), r#"{"version":1,"project":"native","output":"previews","shots":[{"name":"sq010","source":"source.txt","fps":24,"colorspace":"sRGB","command":["/bin/cp","fixture.png","{frames}/frame-0001.png"]}]}"#).expect("manifest");
    let result = Command::new(env!("CARGO_BIN_EXE_shot-runner"))
        .current_dir(dir.path())
        .env_clear()
        .env("PATH", "")
        .args(["run", "shots.json", "--allow-command", "/bin/cp", "--yes"])
        .output()
        .expect("native run without PATH");
    assert_eq!(
        result.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        dir.path()
            .join("previews/sq010/contact-sheet.png")
            .is_file()
    );
}
