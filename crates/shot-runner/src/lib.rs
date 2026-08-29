use image::{DynamicImage, GenericImage, ImageBuffer, Rgba, imageops::FilterType};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct RunnerError {
    pub code: i32,
    pub message: String,
}

impl RunnerError {
    fn manifest(message: impl Into<String>) -> Self {
        Self {
            code: 2,
            message: message.into(),
        }
    }
    fn trust(message: impl Into<String>) -> Self {
        Self {
            code: 3,
            message: message.into(),
        }
    }
    fn renderer(message: impl Into<String>) -> Self {
        Self {
            code: 4,
            message: message.into(),
        }
    }
    fn output(message: impl Into<String>) -> Self {
        Self {
            code: 5,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for RunnerError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub version: u32,
    pub project: String,
    pub output: PathBuf,
    pub shots: Vec<Shot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shot {
    pub name: String,
    pub source: PathBuf,
    pub fps: f64,
    pub colorspace: String,
    pub command: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanItem {
    pub name: String,
    pub executable: String,
    /// The unexpanded, tokenized vector from the manifest.
    pub command: Vec<String>,
    /// The exact argv vector `run` will pass to the renderer when the cache
    /// is cold. Paths are absolute so the vector remains correct when the
    /// renderer runs from the manifest directory.
    pub argv: Vec<String>,
    pub source: String,
    pub source_path: String,
    pub fps: f64,
    pub colorspace: String,
    pub cache_directory: String,
    pub frames_directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileProof {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub schema_version: u32,
    pub tool_version: String,
    pub project: String,
    pub shot: String,
    pub source: String,
    pub source_sha256: String,
    pub manifest_sha256: String,
    pub fps: f64,
    pub colorspace: String,
    pub command: Vec<String>,
    pub cache_key: String,
    pub cache_hit: bool,
    pub created_unix: u64,
    pub frames: Vec<FileProof>,
    pub contact_sheet: FileProof,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunSummary {
    pub project: String,
    pub rendered: usize,
    pub cache_hits: usize,
    pub receipts: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifySummary {
    pub valid: bool,
    pub checked: usize,
    pub mismatches: Vec<String>,
}

pub fn load_manifest(path: &Path) -> Result<(Manifest, String), RunnerError> {
    let bytes = fs::read(path)
        .map_err(|e| RunnerError::manifest(format!("could not read {}: {e}", path.display())))?;
    let manifest: Manifest = serde_json::from_slice(&bytes)
        .map_err(|e| RunnerError::manifest(format!("invalid manifest {}: {e}", path.display())))?;
    validate_manifest(&manifest)?;
    Ok((manifest, hash_bytes(&bytes)))
}

fn validate_manifest(manifest: &Manifest) -> Result<(), RunnerError> {
    if manifest.version != 1 {
        return Err(RunnerError::manifest(format!(
            "unsupported manifest version {}; expected 1",
            manifest.version
        )));
    }
    if manifest.project.trim().is_empty() {
        return Err(RunnerError::manifest("project must not be empty"));
    }
    if manifest.output.as_os_str().is_empty()
        || manifest.output.is_absolute()
        || manifest
            .output
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(RunnerError::manifest(
            "output must be a non-empty relative path",
        ));
    }
    if manifest.shots.is_empty() {
        return Err(RunnerError::manifest(
            "manifest has no shots; add at least one shot",
        ));
    }
    let mut names = BTreeSet::new();
    for shot in &manifest.shots {
        if shot.name.is_empty()
            || !shot
                .name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(RunnerError::manifest(format!(
                "shot name {:?} must use only letters, numbers, - or _",
                shot.name
            )));
        }
        if !names.insert(&shot.name) {
            return Err(RunnerError::manifest(format!(
                "duplicate shot name {}",
                shot.name
            )));
        }
        if shot.source.is_absolute()
            || shot
                .source
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(RunnerError::manifest(format!(
                "source for {} must be relative to the manifest",
                shot.name
            )));
        }
        if !(shot.fps.is_finite() && shot.fps > 0.0) {
            return Err(RunnerError::manifest(format!(
                "fps for {} must be greater than zero",
                shot.name
            )));
        }
        if shot.colorspace.trim().is_empty() {
            return Err(RunnerError::manifest(format!(
                "colorspace for {} must not be empty",
                shot.name
            )));
        }
        if shot.command.is_empty() || shot.command[0].trim().is_empty() {
            return Err(RunnerError::manifest(format!(
                "command for {} must include an executable",
                shot.name
            )));
        }
        if contains_placeholder(&shot.command[0]) {
            return Err(RunnerError::manifest(format!(
                "executable for {} must not contain placeholders",
                shot.name
            )));
        }
    }
    Ok(())
}

pub fn plan(
    path: &Path,
    selected: Option<&str>,
    cache_override: Option<&Path>,
) -> Result<Vec<PlanItem>, RunnerError> {
    let (manifest, _) = load_manifest(path)?;
    let base = manifest_directory(path)?;
    let prepared = prepare_shots(&manifest, &base, selected, cache_override)?;
    Ok(prepared.iter().map(PreparedShot::plan_item).collect())
}

pub fn run(
    path: &Path,
    selected: Option<&str>,
    allowed: &[String],
    confirmed: bool,
    cache_override: Option<&Path>,
) -> Result<RunSummary, RunnerError> {
    let (manifest, manifest_hash) = load_manifest(path)?;
    let base = manifest_directory(path)?;
    let prepared = prepare_shots(&manifest, &base, selected, cache_override)?;
    if !confirmed {
        return Err(RunnerError::trust(
            "refusing to execute manifest commands without --yes; inspect `shot-runner plan` first",
        ));
    }
    for shot in &prepared {
        if !allowed.iter().any(|item| item == &shot.shot.command[0]) {
            return Err(RunnerError::trust(format!(
                "command {:?} is not allowed; pass --allow-command {:?} after reviewing the plan",
                shot.shot.command[0], shot.shot.command[0]
            )));
        }
    }
    let output_root = safe_join(&base, &manifest.output)?;
    let mut summary = RunSummary {
        project: manifest.project.clone(),
        rendered: 0,
        cache_hits: 0,
        receipts: vec![],
    };
    for prepared_shot in prepared {
        let shot = prepared_shot.shot;
        fs::create_dir_all(&prepared_shot.frames_dir)
            .map_err(|e| RunnerError::output(format!("could not create cache: {e}")))?;
        let mut frames = image_files(&prepared_shot.frames_dir)?;
        let cache_hit = !frames.is_empty();
        if !cache_hit {
            let status = Command::new(&prepared_shot.argv[0])
                .args(&prepared_shot.argv[1..])
                .current_dir(&base)
                .status()
                .map_err(|e| {
                    RunnerError::renderer(format!(
                        "could not start {:?} for {}: {e}",
                        prepared_shot.argv[0], shot.name
                    ))
                })?;
            if !status.success() {
                return Err(RunnerError::renderer(format!(
                    "renderer for {} exited with {}",
                    shot.name,
                    status
                        .code()
                        .map_or_else(|| "a signal".into(), |c| c.to_string())
                )));
            }
            frames = image_files(&prepared_shot.frames_dir)?;
            if frames.is_empty() {
                return Err(RunnerError::output(format!(
                    "renderer for {} produced no PNG or JPEG frames in {}",
                    shot.name,
                    prepared_shot.frames_dir.display()
                )));
            }
            summary.rendered += 1;
        } else {
            summary.cache_hits += 1;
        }
        let shot_output = output_root.join(&shot.name);
        let output_frames = shot_output.join("frames");
        fs::create_dir_all(&output_frames)
            .map_err(|e| RunnerError::output(format!("could not create output: {e}")))?;
        let mut proofs = vec![];
        for frame in &frames {
            let file_name = frame
                .file_name()
                .ok_or_else(|| RunnerError::output("frame has no filename"))?;
            let destination = output_frames.join(file_name);
            fs::copy(frame, &destination)
                .map_err(|e| RunnerError::output(format!("could not copy frame: {e}")))?;
            proofs.push(FileProof {
                path: format!("frames/{}", file_name.to_string_lossy()),
                sha256: hash_file(&destination)?,
            });
        }
        let sheet_path = shot_output.join("contact-sheet.png");
        make_contact_sheet(&frames, &sheet_path)?;
        let receipt = Receipt {
            schema_version: 1,
            tool_version: VERSION.into(),
            project: manifest.project.clone(),
            shot: shot.name.clone(),
            source: shot.source.display().to_string(),
            source_sha256: prepared_shot.source_hash,
            manifest_sha256: manifest_hash.clone(),
            fps: shot.fps,
            colorspace: shot.colorspace.clone(),
            command: prepared_shot.argv,
            cache_key: prepared_shot.cache_key,
            cache_hit,
            created_unix: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            frames: proofs,
            contact_sheet: FileProof {
                path: "contact-sheet.png".into(),
                sha256: hash_file(&sheet_path)?,
            },
        };
        let receipt_path = shot_output.join("receipt.json");
        let receipt_bytes =
            serde_json::to_vec_pretty(&receipt).map_err(|e| RunnerError::output(e.to_string()))?;
        fs::write(&receipt_path, receipt_bytes)
            .map_err(|e| RunnerError::output(format!("could not write receipt: {e}")))?;
        summary.receipts.push(receipt_path.display().to_string());
    }
    Ok(summary)
}

pub fn verify(receipt_path: &Path) -> Result<VerifySummary, RunnerError> {
    let bytes = fs::read(receipt_path)
        .map_err(|e| RunnerError::output(format!("could not read receipt: {e}")))?;
    let receipt: Receipt = serde_json::from_slice(&bytes)
        .map_err(|e| RunnerError::output(format!("invalid receipt: {e}")))?;
    let base = receipt_path.parent().unwrap_or_else(|| Path::new("."));
    let mut mismatches = vec![];
    let mut checked = 0;
    for proof in receipt
        .frames
        .iter()
        .chain(std::iter::once(&receipt.contact_sheet))
    {
        checked += 1;
        let candidate = safe_join(base, Path::new(&proof.path))?;
        match hash_file(&candidate) {
            Ok(actual) if actual == proof.sha256 => {}
            Ok(_) => mismatches.push(format!("hash mismatch: {}", proof.path)),
            Err(_) => mismatches.push(format!("missing: {}", proof.path)),
        }
    }
    Ok(VerifySummary {
        valid: mismatches.is_empty(),
        checked,
        mismatches,
    })
}

pub fn starter_manifest() -> &'static str {
    r#"{
  "version": 1,
  "project": "my-animation",
  "output": "previews",
  "shots": [
    {
      "name": "sq010-opening",
      "source": "scenes/opening.blend",
      "fps": 24,
      "colorspace": "sRGB",
      "command": ["blender", "-b", "{source}", "-o", "{frames}/frame_", "-a"]
    }
  ]
}
"#
}

fn select_shots<'a>(
    manifest: &'a Manifest,
    selected: Option<&str>,
) -> Result<Vec<&'a Shot>, RunnerError> {
    match selected {
        None => Ok(manifest.shots.iter().collect()),
        Some(name) => manifest
            .shots
            .iter()
            .find(|s| s.name == name)
            .map(|s| vec![s])
            .ok_or_else(|| RunnerError::manifest(format!("shot {name:?} was not found"))),
    }
}

fn safe_join(base: &Path, relative: &Path) -> Result<PathBuf, RunnerError> {
    if relative.is_absolute()
        || relative
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(RunnerError::manifest(format!(
            "path must stay inside the manifest directory: {}",
            relative.display()
        )));
    }
    Ok(base.join(relative))
}

struct PreparedShot<'a> {
    shot: &'a Shot,
    source_path: PathBuf,
    source_hash: String,
    cache_key: String,
    cache_dir: PathBuf,
    frames_dir: PathBuf,
    argv: Vec<String>,
}

impl PreparedShot<'_> {
    fn plan_item(&self) -> PlanItem {
        PlanItem {
            name: self.shot.name.clone(),
            executable: self.shot.command[0].clone(),
            command: self.shot.command.clone(),
            argv: self.argv.clone(),
            source: self.shot.source.display().to_string(),
            source_path: self.source_path.display().to_string(),
            fps: self.shot.fps,
            colorspace: self.shot.colorspace.clone(),
            cache_directory: self.cache_dir.display().to_string(),
            frames_directory: self.frames_dir.display().to_string(),
        }
    }
}

fn prepare_shots<'a>(
    manifest: &'a Manifest,
    base: &Path,
    selected: Option<&str>,
    cache_override: Option<&Path>,
) -> Result<Vec<PreparedShot<'a>>, RunnerError> {
    let cache_root = cache_directory(base, cache_override)?;
    select_shots(manifest, selected)?
        .into_iter()
        .map(|shot| {
            let source_path = safe_join(base, &shot.source)?;
            if !source_path.exists() {
                return Err(RunnerError::manifest(format!(
                    "source for {} does not exist: {}",
                    shot.name,
                    source_path.display()
                )));
            }
            let source_hash = hash_path(&source_path)?;
            let config =
                serde_json::to_vec(shot).map_err(|e| RunnerError::manifest(e.to_string()))?;
            let cache_key = hash_many(&[source_hash.as_bytes(), &config]);
            let cache_dir = cache_root.join(&cache_key);
            let frames_dir = cache_dir.join("frames");
            let argv = expand_command(
                &shot.command,
                &source_path,
                &frames_dir,
                &shot.name,
                &cache_dir,
            );
            Ok(PreparedShot {
                shot,
                source_path,
                source_hash,
                cache_key,
                cache_dir,
                frames_dir,
                argv,
            })
        })
        .collect()
}

fn manifest_directory(path: &Path) -> Result<PathBuf, RunnerError> {
    let canonical_manifest = path.canonicalize().map_err(|e| {
        RunnerError::manifest(format!(
            "could not resolve manifest {}: {e}",
            path.display()
        ))
    })?;
    canonical_manifest
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            RunnerError::manifest(format!(
                "manifest has no parent directory: {}",
                path.display()
            ))
        })
}

fn cache_directory(base: &Path, cache_override: Option<&Path>) -> Result<PathBuf, RunnerError> {
    match cache_override {
        None => Ok(base.join(".shot-runner/cache")),
        Some(path) if path.is_absolute() => Ok(path.to_path_buf()),
        Some(path) => std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .map_err(|e| RunnerError::manifest(format!("could not resolve cache directory: {e}"))),
    }
}

fn expand_command(
    command: &[String],
    source: &Path,
    frames: &Path,
    shot: &str,
    cache: &Path,
) -> Vec<String> {
    command
        .iter()
        .map(|part| {
            part.replace("{source}", &source.to_string_lossy())
                .replace("{frames}", &frames.to_string_lossy())
                .replace("{shot}", shot)
                .replace("{cache}", &cache.to_string_lossy())
        })
        .collect()
}

fn contains_placeholder(value: &str) -> bool {
    ["{source}", "{frames}", "{shot}", "{cache}"]
        .iter()
        .any(|placeholder| value.contains(placeholder))
}

fn image_files(dir: &Path) -> Result<Vec<PathBuf>, RunnerError> {
    let mut files = fs::read_dir(dir)
        .map_err(|e| RunnerError::output(format!("could not read frame directory: {e}")))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| matches!(e.to_ascii_lowercase().as_str(), "png" | "jpg" | "jpeg"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}

fn make_contact_sheet(frames: &[PathBuf], destination: &Path) -> Result<(), RunnerError> {
    let count = frames.len().min(12);
    let indices: Vec<usize> = if frames.len() <= count {
        (0..count).collect()
    } else {
        (0..count)
            .map(|i| i * (frames.len() - 1) / (count - 1))
            .collect()
    };
    let columns = 4u32.min(count as u32).max(1);
    let rows = (count as u32).div_ceil(columns);
    let (cell_w, cell_h, gutter) = (320u32, 180u32, 8u32);
    let width = columns * cell_w + (columns + 1) * gutter;
    let height = rows * cell_h + (rows + 1) * gutter;
    let mut canvas = DynamicImage::ImageRgba8(ImageBuffer::from_pixel(
        width,
        height,
        Rgba([241, 238, 229, 255]),
    ));
    for (position, index) in indices.iter().enumerate() {
        let frame = image::open(&frames[*index]).map_err(|e| {
            RunnerError::output(format!(
                "could not decode {}: {e}",
                frames[*index].display()
            ))
        })?;
        let thumb = frame.resize(cell_w, cell_h, FilterType::Lanczos3);
        let x =
            gutter + (position as u32 % columns) * (cell_w + gutter) + (cell_w - thumb.width()) / 2;
        let y = gutter
            + (position as u32 / columns) * (cell_h + gutter)
            + (cell_h - thumb.height()) / 2;
        canvas
            .copy_from(&thumb, x, y)
            .map_err(|e| RunnerError::output(format!("could not compose sheet: {e}")))?;
    }
    canvas
        .save(destination)
        .map_err(|e| RunnerError::output(format!("could not write contact sheet: {e}")))
}

fn hash_path(path: &Path) -> Result<String, RunnerError> {
    if path.is_file() {
        return hash_file(path);
    }
    if !path.is_dir() {
        return Err(RunnerError::manifest(format!(
            "source is not a file or directory: {}",
            path.display()
        )));
    }
    let mut files = vec![];
    collect_files(path, path, &mut files)
        .map_err(|e| RunnerError::manifest(format!("could not scan source: {e}")))?;
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let mut hasher = Sha256::new();
    for (relative, full) in files {
        hasher.update(relative.to_string_lossy().as_bytes());
        let mut file = fs::File::open(full).map_err(|e| RunnerError::manifest(e.to_string()))?;
        io::copy(&mut file, &mut HashWriter(&mut hasher))
            .map_err(|e| RunnerError::manifest(e.to_string()))?;
    }
    Ok(hex_digest(hasher.finalize()))
}

fn collect_files(root: &Path, current: &Path, out: &mut Vec<(PathBuf, PathBuf)>) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(root, &path, out)?;
        } else if path.is_file() {
            out.push((path.strip_prefix(root).unwrap_or(&path).to_path_buf(), path));
        }
    }
    Ok(())
}

fn hash_file(path: &Path) -> Result<String, RunnerError> {
    let mut file = fs::File::open(path)
        .map_err(|e| RunnerError::output(format!("could not hash {}: {e}", path.display())))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|e| RunnerError::output(e.to_string()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hex_digest(hasher.finalize()))
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex_digest(h.finalize())
}
fn hash_many(parts: &[&[u8]]) -> String {
    let mut h = Sha256::new();
    for p in parts {
        h.update(p);
    }
    hex_digest(h.finalize())
}
fn hex_digest(bytes: impl AsRef<[u8]>) -> String {
    bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
}

struct HashWriter<'a>(&'a mut Sha256);
impl io::Write for HashWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{TempDir, tempdir};

    fn manifest(command: Vec<String>) -> Manifest {
        Manifest {
            version: 1,
            project: "test".into(),
            output: "previews".into(),
            shots: vec![Shot {
                name: "sq010".into(),
                source: "source.txt".into(),
                fps: 24.0,
                colorspace: "sRGB".into(),
                command,
            }],
        }
    }

    fn preview_fixture(fps: f64, colorspace: &str) -> (TempDir, PathBuf) {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("source.txt"), "source-v1").unwrap();
        DynamicImage::ImageRgba8(ImageBuffer::from_pixel(80, 45, Rgba([20, 30, 40, 255])))
            .save(dir.path().join("fixture.png"))
            .unwrap();
        let mut data = manifest(vec![
            "cp".into(),
            "fixture.png".into(),
            "{frames}/frame-0001.png".into(),
        ]);
        data.shots[0].source = "source.txt".into();
        data.shots[0].fps = fps;
        data.shots[0].colorspace = colorspace.into();
        let path = dir.path().join("shots.json");
        fs::write(&path, serde_json::to_vec_pretty(&data).unwrap()).unwrap();
        (dir, path)
    }

    #[test]
    fn rejects_parent_paths() {
        let mut m = manifest(vec!["renderer".into()]);
        m.output = "../away".into();
        assert!(validate_manifest(&m).is_err());
    }

    #[test]
    fn rejects_placeholder_executables() {
        let invalid = manifest(vec!["{source}".into()]);
        assert!(validate_manifest(&invalid).is_err());
    }

    #[test]
    fn requires_confirmation_before_execution() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("source.txt"), "x").unwrap();
        let path = dir.path().join("shots.json");
        fs::write(
            &path,
            serde_json::to_vec(&manifest(vec!["renderer".into()])).unwrap(),
        )
        .unwrap();
        assert_eq!(
            run(&path, None, &["renderer".into()], false, None)
                .unwrap_err()
                .code,
            3
        );
    }

    #[test]
    fn documented_plan_is_parseable() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("shots.json");
        fs::create_dir_all(dir.path().join("scenes")).unwrap();
        fs::write(dir.path().join("scenes/opening.blend"), "fixture").unwrap();
        fs::write(&path, starter_manifest()).unwrap();
        let items = plan(&path, None, None).unwrap();
        assert_eq!(items[0].name, "sq010-opening");
        assert_eq!(items[0].executable, "blender");
        assert_eq!(items[0].command[0], "blender");
        assert_eq!(items[0].argv[0], "blender");
    }

    #[test]
    fn contact_sheet_is_deterministic() {
        let dir = tempdir().unwrap();
        let frame = dir.path().join("frame.png");
        DynamicImage::ImageRgba8(ImageBuffer::from_pixel(64, 32, Rgba([10, 20, 30, 255])))
            .save(&frame)
            .unwrap();
        let a = dir.path().join("a.png");
        let b = dir.path().join("b.png");
        make_contact_sheet(std::slice::from_ref(&frame), &a).unwrap();
        make_contact_sheet(&[frame], &b).unwrap();
        assert_eq!(hash_file(&a).unwrap(), hash_file(&b).unwrap());
    }

    #[test]
    fn run_writes_copied_frames_contact_sheet_and_receipt() {
        let (dir, path) = preview_fixture(24.0, "sRGB");
        let first = run(&path, None, &["cp".into()], true, None).unwrap();
        assert_eq!(first.rendered, 1);
        assert_eq!(first.cache_hits, 0);
        let receipt = dir.path().join("previews/sq010/receipt.json");
        assert!(
            dir.path()
                .join("previews/sq010/frames/frame-0001.png")
                .is_file()
        );
        assert!(
            dir.path()
                .join("previews/sq010/contact-sheet.png")
                .is_file()
        );
        assert!(receipt.is_file());
        assert!(verify(&receipt).unwrap().valid);
    }

    #[test]
    fn second_unchanged_run_reuses_local_cache() {
        let (_dir, path) = preview_fixture(24.0, "sRGB");
        let first = run(&path, None, &["cp".into()], true, None).unwrap();
        assert_eq!(first.rendered, 1);
        let second = run(&path, None, &["cp".into()], true, None).unwrap();
        assert_eq!(second.rendered, 0);
        assert_eq!(second.cache_hits, 1);
    }

    #[test]
    fn receipt_records_verified_hashes_fps_and_colorspace() {
        let (dir, path) = preview_fixture(23.976, "Display P3");
        run(&path, None, &["cp".into()], true, None).unwrap();

        let receipt_path = dir.path().join("previews/sq010/receipt.json");
        let receipt: Receipt = serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
        let written_frame = dir.path().join("previews/sq010/frames/frame-0001.png");
        let contact_sheet = dir.path().join("previews/sq010/contact-sheet.png");

        assert_eq!(receipt.fps, 23.976);
        assert_eq!(receipt.colorspace, "Display P3");
        assert_eq!(
            receipt.source_sha256,
            hash_file(&dir.path().join("source.txt")).unwrap()
        );
        assert_eq!(receipt.frames.len(), 1);
        assert_eq!(receipt.frames[0].sha256, hash_file(&written_frame).unwrap());
        assert_eq!(
            receipt.contact_sheet.sha256,
            hash_file(&contact_sheet).unwrap()
        );
        let verified = verify(&receipt_path).unwrap();
        assert!(verified.valid);
        assert_eq!(verified.checked, 2);
    }
}
