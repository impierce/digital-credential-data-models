use crate::tests::assert_eq_json_value;
use types_ob_v3::prelude::*; 
use std::fs::File;

#[test]
fn basic_achievement_credential() {
    // Testing if serialization and deserialization between the OBv3 examples and our rust code works as needed.

    assert_eq_json_value::<AchievementCredential>("tests/obv3_json_examples/basic_achievement_credential.json");

    // Next, the builders are tested against the OBv3 examples

    let criteria_builder: Criteria = CriteriaBuilder::default()
        .narrative("Team members are nominated for this badge by their peers and recognized upon review by Example Corp management.".to_string())
        .try_into()
        .unwrap();

    let achievement_builder: Achievement = AchievementBuilder::default()
        .id("https://example.com/achievements/21st-century-skills/teamwork".to_string())
        .type_("Achievement")
        .criteria(criteria_builder)
        .name("Teamwork")
        .description("This badge recognizes the development of the capacity to collaborate within a group environment.")
        .try_into()
        .unwrap();

    let achievement_subject_builder: AchievementSubject = AchievementSubjectBuilder::default()
        .id("did:example:ebfeb1f712ebc6f1c276e12ec21".to_string())
        .type_("AchievementSubject")
        .achievement(achievement_builder)
        .try_into()
        .unwrap();

    let issuer_builder: Profile = ProfileBuilder::default()
        .id("https://example.com/issuers/876543")
        .type_("Profile")
        .name("Example Corp".to_string())
        .try_into()
        .unwrap();

    let basic_achievement_credential: AchievementCredential = AchievementCredentialBuilder::default()
        .context(vec![
            "https://www.w3.org/2018/credentials/v1",
            "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json",
        ])
        .credential_subject(&achievement_subject_builder)
        .id("http://example.com/credentials/3527")
        .name("Teamwork Badge")
        .type_(AchievementCredentialType::from(vec![
            "VerifiableCredential",
            "OpenBadgeCredential",
        ]))
        .issuance_date("2010-01-01T00:00:00Z")
        .issuer(issuer_builder)
        .try_into()
        .unwrap();

    // Here we test the built struct against the struct deserialized from the example .json file.

    let file = File::open("tests/obv3_json_examples/basic_achievement_credential.json").expect("Failed to open file");
    let basic_achievement_cred_from_file: AchievementCredential =
        serde_json::from_reader(&file).expect("Couldn't read from file");

    assert_eq!(basic_achievement_credential, basic_achievement_cred_from_file);

    // Here we test the built struct converted to a json_value against the json_value deserialized from the example .json file

    let file = File::open("tests/obv3_json_examples/basic_achievement_credential.json").expect("Failed to open file");
    let json_value_from_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(
        serde_json::to_value(basic_achievement_credential).unwrap(),
        json_value_from_file
    );
}
