use crate::{
    json_schema::{
        achievement_credential::AchievementCredential,
        endorsement::EndorsementCredential
    }
};
use std::fs::File;
use std::path::Path;
use serde::{de::DeserializeOwned, Serialize};

pub fn json_example<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("file does not exist");
    serde_json::from_reader::<_, T>(file).expect("could not parse json")
}

fn assert_eq_json_value<T>(_path: &str)
where
    T: DeserializeOwned + PartialEq + Serialize
    {
    let json_achievement_credential: T =
        json_example(_path);
    let json_v_achievement_cred: serde_json::Value =
        serde_json::to_value(json_achievement_credential).expect("to_value");

    let file = File::open(_path)
        .expect("Failed to open file");
    let json_v_file: serde_json::Value =
        serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(json_v_achievement_cred, json_v_file); 
}

// The following tests all examples of the OBv3 website: https://www.imsglobal.org/spec/ob/v3p0#examples-0
#[cfg(test)]
#[test]
fn test_obv3_examples() {

    // The following tests the Json files deserialized into Json_Values against the Json files Deserialized completely into Rust structs,
    // and then Serialized back into Json_Values. This proves are Rust translations works.
    // Valid schemas have to be tested as Json_Values, as the order of struct fields is explicitly made randomly 
    // in the examples of OBv3.
    assert_eq_json_value::<AchievementCredential>("src/tests/json_examples/basic_achievement_credential.json");
    assert_eq_json_value::<AchievementCredential>("src/tests/json_examples/full_achievement_credential.json");
    assert_eq_json_value::<EndorsementCredential>("src/tests/json_examples/endorsement_credential.json");
    assert_eq_json_value::<AchievementCredential>("src/tests/json_examples/alignment.json");
    assert_eq_json_value::<AchievementCredential>("src/tests/json_examples/alignment2.json");
    assert_eq_json_value::<AchievementCredential>("src/tests/json_examples/skill_assertion.json");
    assert_eq_json_value::<AchievementCredential>("src/tests/json_examples/skill_assertion2.json");

}
