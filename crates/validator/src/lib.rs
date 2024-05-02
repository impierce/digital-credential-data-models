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

    fn validate_file(filename: &str) {
        let manifest_dir = manifest_dir();

        let file_path = format!("elm-requests/{}", filename);
        let result = validate(manifest_dir.join(file_path));

        if let Some(err) = result.as_ref().err() {
            panic!("{}", err);
        }

        if let Ok(result) = result {
            assert!(result.valid_shacl);
            assert!(result.rust_object.is_ok());
        }
    }

    #[test]
    fn test_bengales_diploma() {
        validate_file("bengales-highschool-diploma.json");
    }

    #[test]
    fn test_sample_request() {
        validate_file("credential-sample.json");
    }

    #[test]
    fn test_digicomp_generic() {
        validate_file("digicomp-generic.json");
    }

    #[test]
    fn test_rntuo_credential() {
        validate_file("diploma-rntuo-credential.json");
    }

    #[test]
    fn test_francisco_cruz() {
        validate_file("francisco-cruz-argudo-cert-of-completion.json");
    }
}
