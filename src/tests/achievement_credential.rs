#[cfg(test)]
#[test]
fn test_achievement_credential() {
    use crate::{
        json_schema::{
            achievement::{Achievement, Criteria},
            achievement_credential::AchievementCredential,
            achievement_subject::AchievementSubject,
            general::{self, Context},
        },
        tests::json_example,
    };
    use std::fs::File;
    use std::io::Read;

    let json_achievement_credential: AchievementCredential =
        json_example("src/tests/json_examples/full_achievement_credential.json");
    let json_v_achievement_cred: serde_json::Value =
        serde_json::to_value(&json_achievement_credential).expect("to_value");
    let json_str_achievement_cred =
        serde_json::to_string_pretty(&json_achievement_credential).expect("str");

    let file = File::open("src/tests/json_examples/full_achievement_credential.json")
        .expect("Failed to open file");
    let json_v_file: serde_json::Value =
        serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(json_v_achievement_cred, json_v_file); //volgorde niet belangrijk

    let mut file2 = File::open("src/tests/json_examples/full_achievement_credential.json")
        .expect("Failed to open file");
    let mut file_str = String::new();
    file2.read_to_string(&mut file_str).expect("file_str");

    println!(
        "{}\n\n------------------\n\n{}",
        json_str_achievement_cred, file_str
    );
    assert_eq!(json_str_achievement_cred, file_str);

    // let _achievement_credential: AchievementCredential = AchievementCredential::builder()
    //     .context(vec![
    //             "https://www.w3.org/2018/credentials/examples/v1".into(),
    //     ])
    //     .credential_subject(
    //         AchievementSubject::builder()
    //         .achievement(Achievement::builder().criteria(Criteria::builder())),
    //     )
    //     .try_into()
    //     .unwrap();

    //     context: vec!("https://www.w3.org/2018/credentials/v1".to_string(),
    //     "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".to_string()),
    //     id: "http://example.com/credentials/3527".to_string(),
    //     type_: ["VerifiableCredential", "OpenBadgeCredential"],
    //     credential_subject: todo!(),
    //     issuance_date: todo!(),
    //     issuer: todo!(),
    //     name: todo!(),
    //     ..Default::default()
    // };

    // println!("Constructed Achievement Credential: {:?}", _achievement_credential);

    // assert_eq!(json_achievement_credential, _achievement_credential);
}
