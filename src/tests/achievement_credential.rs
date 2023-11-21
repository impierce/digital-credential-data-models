#[cfg(test)]
#[test]
fn test_achievement_credential() {
    use crate::{json_schema::{achievement::{Achievement, Criteria}, achievement_credential::AchievementCredential, general::{Context, self}, achievement_subject::AchievementSubject}, tests::json_example};

    let json_achievement_credential: AchievementCredential = json_example("src/tests/json_examples/achievement_credential.json");

    let _achievement_credential: AchievementCredential = AchievementCredential::builder()
        .context(vec![
                "https://www.w3.org/2018/credentials/examples/v1".into(),
        ])
        .credential_subject(
            AchievementSubject::builder()
            .achievement(Achievement::builder().criteria(Criteria::builder())),
        )
        .try_into()
        .unwrap();

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

    println!("Constructed Achievement Credential: {:?}", _achievement_credential);
    
    assert_eq!(json_achievement_credential, _achievement_credential);
}
