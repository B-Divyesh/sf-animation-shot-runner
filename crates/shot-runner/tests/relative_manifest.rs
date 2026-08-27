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
