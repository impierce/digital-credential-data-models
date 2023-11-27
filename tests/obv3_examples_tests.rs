use openbadges::json_schema::{
    achievement::{Achievement, AchievementBuilder, Criteria, CriteriaBuilder},
    achievement_credential::{AchievementCredential, AchievementCredentialBuilder, AchievementCredentialType},
    achievement_subject::{AchievementSubject, AchievementSubjectBuilder},
    endorsement::EndorsementCredential,
    profile::{Profile, ProfileBuilder},
};

use serde::{de::DeserializeOwned, Serialize};
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

// The following tests all examples of the OBv3 website: https://www.imsglobal.org/spec/ob/v3p0#examples-0

#[test]
fn test_obv3_examples() {
    // The following function tests the Json files deserialized into Json_Values against the Json files Deserialized completely into Rust structs,
    // and then Serialized back into Json_Values. This proves are Rust translations works.
    // Valid schemas have to be tested as Json_Values, as the order of struct fields is explicitly made random
    // in the examples of OBv3.

    assert_eq_json_value::<AchievementCredential>("tests/json_examples/basic_achievement_credential.json");
    assert_eq_json_value::<AchievementCredential>("tests/json_examples/full_achievement_credential.json");
    assert_eq_json_value::<EndorsementCredential>("tests/json_examples/endorsement_credential.json");
    assert_eq_json_value::<AchievementCredential>("tests/json_examples/alignment.json");
    assert_eq_json_value::<AchievementCredential>("tests/json_examples/alignment2.json");
    assert_eq_json_value::<AchievementCredential>("tests/json_examples/skill_assertion.json");
    assert_eq_json_value::<AchievementCredential>("tests/json_examples/skill_assertion2.json");

    //

    let criteria_builder: Criteria = CriteriaBuilder::default()
        .id("ID".to_string())
        .narrative("Nara".to_string())
        .try_into()
        .unwrap();

    println!("\n\ncriteria_builder: {:?}", criteria_builder);

    let achievement_builder: Achievement = AchievementBuilder::default()
        .id("https://example.com/achievements/21st-century-skills/teamwork".to_string())
        .type_("Achievement")
        .criteria(criteria_builder)
        .name("Teamwork")
        .description("This badge recognizes the development of the capacity to collaborate within a group environment.")
        .try_into()
        .unwrap();

    println!("\n\nachievement_builder: {:?}", achievement_builder);

    let achievement_subject_builder: AchievementSubject = AchievementSubjectBuilder::default()
        .id("did:example:ebfeb1f712ebc6f1c276e12ec21".to_string())
        .type_("AchievementSubject")
        .achievement(achievement_builder)
        .try_into()
        .unwrap();

    println!("\n\nachievement_subject_builder: {:?}", &achievement_subject_builder);

    let issuer_builder: Profile = ProfileBuilder::default()
        .id("https://example.com/issuers/876543")
        .type_("Profile")
        .name("Example Corp".to_string())
        .try_into()
        .unwrap();

    println!("\n\nissuer_builder: {:?}", &issuer_builder);

    let achievement_credential_builder: AchievementCredential = AchievementCredentialBuilder::default()
        .context(vec![
            "https://www.w3.org/2018/credentials/v1".into(),
            "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
        ])
        .credential_subject(&achievement_subject_builder)
        .id("http://example.com/credentials/3527")
        .name("Teamwork Badge")
        .type_(AchievementCredentialType::from(vec![
            "VerifiableCredential",
            "OpenBadgeCredential",
        ]))
        .issuance_date(
            "2010-01-01T00:00:00Z"
                .parse::<chrono::DateTime<chrono::offset::Utc>>()
                .unwrap(),
        )
        .issuer(issuer_builder)
        .try_into()
        .unwrap();

    println!(
        "\n\nachievement_credential_builder: {:?}",
        achievement_credential_builder
    );
}
