use crate::tests::assert_eq_json_value;
use openbadges::json_schema::{
    achievement::{Achievement, AchievementBuilder, Criteria, CriteriaBuilder},
    achievement_credential::{AchievementCredential, AchievementCredentialBuilder, AchievementCredentialType},
    achievement_subject::{AchievementSubject, AchievementSubjectBuilder},
    profile::{Profile, ProfileBuilder},
};
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

    let file = File::open("tests/obv3_json_examples/basic_achievement_credential.json").expect("Failed to open file");
    let json_v_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(serde_json::to_value(achievement_credential_builder).unwrap(), json_v_file);
}
