use std::{io, path::PathBuf, process::Command};

use crate::manifest_dir;

#[derive(Clone, Copy, Debug)]
pub struct ValidateRequest {
    pub valid_shacl: bool,
    pub valid_rust: bool,
}

impl ValidateRequest {
    pub fn new(json_file: PathBuf) -> io::Result<Self> {
        let manifest_dir = manifest_dir();
        let python_dir = manifest_dir.join("src/py-shacl-validator/");

        activate_venv(&python_dir)?;

        let valid_shacl = validate_shacl(&json_file, &python_dir)?;
        let valid_rust = validate_rust(&json_file, &python_dir)?;

        Ok(Self {
            valid_shacl,
            valid_rust,
        })
    }
}

fn activate_venv(python_dir: &PathBuf) -> io::Result<()> {
    let out = Command::new("source")
        .current_dir(python_dir)
        .arg("venv/bin/activate")
        .output()?;

    if !out.status.success() {
        panic!("Not a valid command source");
    }

    Ok(())
}

fn validate_shacl(json_file: &PathBuf, python_dir: &PathBuf) -> io::Result<bool> {
    let out = Command::new("python")
        .current_dir(python_dir)
        .args(["--input-file", json_file.to_str().unwrap()])
        .output()?;

    Ok(out.status.success())
}

fn validate_rust(json_file: &PathBuf, python_dir: &PathBuf) -> io::Result<bool> {
    // TODO
    Ok(true)
}
