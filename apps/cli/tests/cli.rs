use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Self {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!("ludeforge-cli-{}-{stamp}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run(args: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ludeforge")).args(args).output().unwrap()
}

#[test]
fn cli_validation_normalization_and_exclusive_create() {
    let dir = Scratch::new();
    let input = dir.0.join("input.json");
    let output = dir.0.join("normalized.json");
    let bytes = include_bytes!("../../../examples/lantern-academy/project.json");
    fs::write(&input, bytes).unwrap();
    let result = run(&[Path::new("validate"), &input]);
    assert!(result.status.success(), "{:?}", result);
    assert!(run(&[Path::new("normalize"), &input, &output]).status.success());
    let normalized = fs::read(&output).unwrap();
    assert!(ludeforge_core::load(&normalized).is_ok());
    assert_eq!(fs::read(&input).unwrap(), bytes);
    assert!(!run(&[Path::new("normalize"), &input, &input]).status.success());
    assert!(!run(&[Path::new("normalize"), &input, &output]).status.success());
    assert_eq!(fs::read(&output).unwrap(), normalized);
    assert_eq!(fs::read(&input).unwrap(), bytes);
    let refs = run(&[Path::new("references"), &input]);
    assert!(refs.status.success());
    assert!(serde_json::from_slice::<serde_json::Value>(&refs.stdout).unwrap().is_array());
}

#[test]
fn invalid_validation_leaves_no_output() {
    let dir = Scratch::new();
    let input = dir.0.join("invalid.json");
    let output = dir.0.join("never-created.json");
    fs::write(&input, b"{}").unwrap();
    let result = run(&[Path::new("normalize"), &input, &output]);
    assert_eq!(result.status.code(), Some(1));
    assert!(!output.exists());
    let diagnostics: serde_json::Value = serde_json::from_slice(&result.stderr).unwrap();
    assert_eq!(diagnostics[0]["code"], "FORMAT_VERSION");
    assert_eq!(run(&[]).status.code(), Some(2));
}
