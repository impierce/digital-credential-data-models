use std::{fs, io, path::PathBuf, process::Command};

use serde_json::Error;
use types_elm_v3::codegen::EuropeanDigitalCredential;

use crate::manifest_dir;

#[derive(Clone, Copy, Debug)]
pub struct ValidateRequest {
    pub valid_shacl: bool,
    pub valid_rust: bool,
}

impl ValidateRequest {
    pub fn new(json_file: PathBuf) -> io::Result<Self> {
        let valid_shacl = validate_shacl(&json_file)?;
        let valid_rust = validate_rust(&json_file)?;

        Ok(Self {
            valid_shacl,
            valid_rust,
        })
    }
}

fn validate_shacl(json_file: &PathBuf) -> io::Result<bool> {
    let manifest_dir = manifest_dir();
    let cwd = manifest_dir.join("src/shacl-validator/");
    let python_bin = manifest_dir.join("src/shacl-validator/venv/bin/python");

    let install_pip = Command::new(&python_bin)
        .current_dir(&cwd)
        .args(["-m", "ensurepip", "--upgrade"])
        .output()?;

    if !install_pip.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Can't install pip"));
    }

    let install_dep = Command::new(&python_bin)
        .current_dir(&cwd)
        .args(["-m", "pip", "install", "-r", "requirements.txt", "--upgrade"])
        .output()?;

    if !install_dep.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Can't install python dependencies"));
    }

    let out = Command::new(python_bin)
        .current_dir(cwd)
        .args(["main.py", "--input-file", json_file.to_str().unwrap()])
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&out.stderr));

    Ok(out.status.success())
}

fn validate_rust(json_file: &PathBuf) -> io::Result<bool> {
    // TODO
    let file_str = fs::read_to_string(json_file)?;

    let value: Result<EuropeanDigitalCredential, Error> = serde_json::from_str(&file_str);

    if let Err(err) = &value {
        eprintln!("Error: {err:?}");
    }

    assert!(value.is_ok());

    Ok(true)
}
