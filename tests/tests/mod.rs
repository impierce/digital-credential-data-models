// The following tests all examples of the OBv3 website: https://www.imsglobal.org/spec/ob/v3p0#examples-0
mod basic_achievement_credential;
mod full_achievement_credential;
mod endorsement_credential;
mod alignment_case;
mod alignment_credential_engine;
mod skill_assertion_case;
mod skill_assertion_credential_engine;

// Below are functions defined for use across all OBv3 example tests.

use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;
use std::path::Path;

// This function deserializes a .json file into the desired struct <T>
pub fn json_example<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("file does not exist");
    serde_json::from_reader::<_, T>(file).expect("could not parse json")
}

// The following function tests the Json files deserialized into Json_Values against the Json files Deserialized completely into Rust structs,
// and then Serialized back into Json_Values. This proves are Rust translations works.
// Valid schemas have to be tested as Json_Values, as the order of struct fields is explicitly made random
// in the examples of OBv3.
fn assert_eq_json_value<T>(_path: &str)
where
    T: DeserializeOwned + PartialEq + Serialize,
{
    let json_achievement_credential: T = json_example(_path);
    let json_v_achievement_cred: serde_json::Value =
        serde_json::to_value(json_achievement_credential).expect("to_value");

    let file = File::open(_path).expect("Failed to open file");
    let json_v_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(json_v_achievement_cred, json_v_file);
}
