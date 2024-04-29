use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
    process::Command,
};
use types_elm_v3::codegen;

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

    let out = Command::new(python_bin)
        .current_dir(cwd)
        .args(["main.py", "--input-file", json_file.to_str().unwrap()])
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&out.stderr));

    Ok(out.status.success())
}

fn validate_rust(json_file: &PathBuf) -> io::Result<bool> {
    let file = File::open(json_file)?;
    let rdr = BufReader::new(file);

    let result: serde_json::Result<codegen::EuropassEdcCredential> = serde_json::from_reader(rdr);

    if let Err(err) = &result {
        eprintln!("Error: {err:?}");
    }

    Ok(result.is_ok())
}
