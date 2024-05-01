use std::{fs, io, path::PathBuf};

use validator::ValidateRequest;

mod validator;

pub fn validate(file: PathBuf) -> io::Result<ValidateRequest> {
    if !file.is_file() {
        panic!("Not a valid file!");
    }

    let is_json = file.extension().map(|e| e == "json" || e == "jsonld").unwrap_or(false);

    if !is_json {
        panic!("Not a json(ld) file");
    }

    let absolute = fs::canonicalize(&file)?;

    ValidateRequest::new(absolute)
}

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[cfg(test)]
mod tests {
    use env_logger::Env;

    use super::*;

    #[ctor::ctor]
    fn init() {
        let env = Env::default()
            .filter_or("DCDM_LOG_LEVEL", "debug")
            .write_style_or("DCDM_LOG_STYLE", "always");

        env_logger::init_from_env(env);
    }

    #[test]
    #[should_panic]
    fn no_file() {
        let _ = validate(PathBuf::new());
    }

    #[test]
    #[should_panic]
    fn no_json_file() {
        let _ = validate(PathBuf::from("lib.rs"));
    }

    #[test]
    fn test_sample_request() {
        let manifest_dir = manifest_dir();
        let result = validate(manifest_dir.join("elm-requests/credential-sample.json"));

        assert!(result.is_ok());

        if let Ok(result) = result {
            assert!(result.valid_shacl);
            assert!(result.valid_rust);
        }
    }

    #[test]
    fn test_bengales_diploma() {
        let manifest_dir = manifest_dir();
        let result = validate(manifest_dir.join("elm-requests/bengales-highschool-diploma.json"));

        assert!(result.is_ok());

        if let Ok(result) = result {
            assert!(result.valid_shacl);
            assert!(result.valid_rust);
        }
    }
}
