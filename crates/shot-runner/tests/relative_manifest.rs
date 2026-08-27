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
