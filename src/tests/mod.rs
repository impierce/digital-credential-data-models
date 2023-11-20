mod achievement_credential;

use serde::de::DeserializeOwned;
use std::fs::File;
use std::path::Path;

pub fn json_example<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("file does not exist");
    serde_json::from_reader::<_, T>(file).expect("could not parse json")
}
