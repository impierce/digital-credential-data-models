use log::{error, info};
use serde::Deserialize;
use std::{fs, io, path::PathBuf, process::Command};
use types_elm_v3::EuropassEdcCredential;

use crate::manifest_dir;

#[derive(Debug)]
pub struct ValidateRequest {
    pub valid_shacl: bool,
    pub rust_object: io::Result<EuropassEdcCredential>,
}

impl ValidateRequest {
    pub fn new(json_file: PathBuf) -> io::Result<Self> {
        let valid_shacl = validate_shacl(&json_file)?;
        let rust_object = validate_rust(&json_file);

        Ok(Self {
            valid_shacl,
            rust_object,
        })
    }
}

pub fn validate_shacl(json_file: &PathBuf) -> io::Result<bool> {
    let manifest_dir = manifest_dir();
    let cwd = manifest_dir.join("src/shacl-validator/");
    let python_bin = manifest_dir.join("src/shacl-validator/venv/bin/python");

    let out = Command::new(python_bin)
        .current_dir(cwd)
        .args(["main.py", "--input-file", json_file.to_str().unwrap()])
        .output()?;

    if !out.status.success() {
        error!("{}", String::from_utf8_lossy(&out.stderr));
    } else {
        info!("{}", String::from_utf8_lossy(&out.stdout));
    }

    Ok(out.status.success())
}

fn validate_rust(json_file: &PathBuf) -> io::Result<EuropassEdcCredential> {
    let json = fs::read_to_string(json_file)?;
    let mut deserializer = serde_json::Deserializer::from_str(&json);
    deserializer.disable_recursion_limit();

    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

    let credential = EuropassEdcCredential::deserialize(deserializer);

    if let Err(err) = &credential {
        eprintln!("Error: {err:?}");
    }

    Ok(credential?)
}
