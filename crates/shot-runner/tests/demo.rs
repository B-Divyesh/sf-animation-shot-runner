use serde_json::Value;
use std::{fs, process::Command};
use tempfile::tempdir;

#[test]
fn demo_renders_five_bundled_shots_in_a_new_temp_folder() {
    let output = Command::new(env!("CARGO_BIN_EXE_shot-runner"))
        .args(["--json", "demo"])
        .output()
        .expect("run bundled demo");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).expect("demo JSON");
    assert_eq!(value["rendered"], 5);
    assert_eq!(value["cache_hits_on_repeat"], 5);
    let directory = value["directory"].as_str().expect("demo directory");
    for shot in [
        "sq010-arrival",
        "sq020-door",
        "sq030-crossing",
        "sq040-turn",
        "sq050-exit",
    ] {
        let output = std::path::Path::new(directory).join("previews").join(shot);
        assert!(
            output.join("frames/frame-0001.png").is_file(),
            "{shot} frame"
        );
        assert!(
            output.join("contact-sheet.png").is_file(),
            "{shot} contact sheet"
        );
        assert!(output.join("receipt.json").is_file(), "{shot} receipt");
    }
    fs::remove_dir_all(directory).expect("remove only demo temp directory");
}

#[test]
fn demo_receipt_detects_a_tampered_sample_frame() {
    let output = Command::new(env!("CARGO_BIN_EXE_shot-runner"))
        .args(["--json", "demo"])
        .output()
        .expect("run bundled demo");
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).expect("demo JSON");
    assert_eq!(
        value["cache_hits_on_repeat"], 5,
        "the repeated demo run reuses all five cached frames"
    );
    let directory = std::path::PathBuf::from(value["directory"].as_str().expect("demo directory"));
    let receipt = directory.join("previews/sq010-arrival/receipt.json");
    let valid = Command::new(env!("CARGO_BIN_EXE_shot-runner"))
        .args(["--json", "verify", receipt.to_str().expect("utf8 receipt")])
        .output()
        .expect("verify unchanged sample");
    assert!(valid.status.success());
    let verified: Value = serde_json::from_slice(&valid.stdout).expect("verify JSON");
    assert_eq!(verified["valid"], true);
    assert_eq!(
        verified["checked"], 2,
        "the sample receipt covers its frame and contact sheet"
    );
    fs::write(
        directory.join("previews/sq010-arrival/frames/frame-0001.png"),
        b"changed",
    )
    .expect("tamper frame");
    let verify = Command::new(env!("CARGO_BIN_EXE_shot-runner"))
        .args(["verify", receipt.to_str().expect("utf8 receipt")])
        .output()
        .expect("verify tampered sample");
    assert_eq!(verify.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&verify.stderr).contains("hash mismatch"));
    fs::remove_dir_all(directory).expect("remove only demo temp directory");
}

#[test]
fn demo_leaves_the_callers_project_files_unchanged() {
    let caller = tempdir().expect("temporary caller project");
    let sentinel = caller.path().join("do-not-touch.scene");
    fs::write(&sentinel, "original caller project data").expect("sentinel");
    let output = Command::new(env!("CARGO_BIN_EXE_shot-runner"))
        .current_dir(caller.path())
        .args(["--json", "demo"])
        .output()
        .expect("run bundled demo");
    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(&sentinel).expect("sentinel remains"),
        "original caller project data"
    );
    assert_eq!(
        fs::read_dir(caller.path()).expect("caller files").count(),
        1,
        "demo writes no caller files"
    );
    let value: Value = serde_json::from_slice(&output.stdout).expect("demo JSON");
    let directory = std::path::PathBuf::from(value["directory"].as_str().expect("demo directory"));
    assert!(
        directory.starts_with(std::env::temp_dir()),
        "demo output is under the system temp directory"
    );
    fs::remove_dir_all(directory).expect("remove only demo temp directory");
}
