use crate::tests::assert_eq_json_value;
use openbadges::{
    achievement::{AchievementBuilder, AchievementType, CriteriaBuilder},
    achievement_credential::{
        AchievementCredential, AchievementCredentialBuilder, AchievementCredentialType, CredentialSchemaBuilder,
    },
    achievement_subject::AchievementSubjectBuilder,
    alignment::{AlignmentBuilder, AlignmentTargetType},
    general::ImageBuilder,
    profile::ProfileBuilder,
};
use std::{fs::File, str::FromStr};

#[test]
fn skill_assertion_credential_engine() {
    // Testing if serialization and deserialization between the OBv3 examples and our rust code works as needed.

    assert_eq_json_value::<AchievementCredential>("tests/obv3_json_examples/skill_assertion_credential_engine.json");

    // Next, the builders are tested against the OBv3 examples

    let skill_assertion_credential_engine: AchievementCredential = AchievementCredentialBuilder::default()
    .context(vec![
        "https://www.w3.org/2018/credentials/v1",
		"https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json",
		"https://purl.imsglobal.org/spec/ob/v3p0/extensions.json"
    ])
    .id("http://1edtech.edu/credentials/3732")
    .type_(AchievementCredentialType::from(vec!["VerifiableCredential", "OpenBadgeCredential"]))
    .name("Solve and graph linear equations and inequalities")
    .credential_subject(
        AchievementSubjectBuilder::default()
        .id("did:example:ebfeb1f712ebc6f1c276e12ec21".to_string())
        .type_("AchievementSubject",)
        .achievement(
            AchievementBuilder::default()
            .id("https://example.com/achievements/math/linear-1")
            .type_("Achievement")
            .alignment(
                vec![AlignmentBuilder::default()
                .type_("Alignment")
                .target_code("ce-6369c51f-4d86-4592-a761-8b32ae70a045".to_string())
                .target_framework("Ivy Tech Community College of Indiana, MATH 135, FINITE MATH".to_string())
                .target_name("Solve and graph linear equations and inequalities")
                .target_type(AlignmentTargetType::from_str("ceasn:Competency").unwrap())
                .target_url("https://credentialfinder.org/competency/ce-6369c51f-4d86-4592-a761-8b32ae70a045")
            ])
            .achievement_type(AchievementType::from_str("Competency").unwrap())
            .creator(
                ProfileBuilder::default()
                .id("https://example.com/issuers/123767")
                .type_("Profile")
                .name("Example Industry Group".to_string())
                .url("https://example.com".to_string())
                .description("Example Industry Group is a consortium of luminaries who publish skills data for common usage.".to_string())
                .email("info@exammple.com".to_string())
            )
            .criteria({
                CriteriaBuilder::default()
                .narrative("Learners must demonstrate understanding of linear algebra and graphic representation of linear equations.".to_string())
            })
            .description("This achievement represents developing capability to solve and graph linear equations and inequalities")
            .image({
                ImageBuilder::default()
                .id("https://example.com/achievements/math/linear-1/image")
                .type_("Image")
                .caption("A line, sloping upward optimistically".to_string())
            })
            .name("Linear equations and inequalities")
        )
    )
    .issuer({
        ProfileBuilder::default()
        .id("https://1edtech.edu/issuers/565049")
        .type_("Profile")
        .name("1EdTech University".to_string())
        .url("https://1edtech.edu".to_string())
        .phone("1-222-333-4444".to_string())
        .description("1EdTech University provides online degree programs.".to_string())
        .image({
            ImageBuilder::default()
            .id("https://1edtech.edu/logo.png")
            .type_("Image")
            .caption("1EdTech University logo".to_string())
        })
        .email("registrar@1edtech.edu".to_string())
    })
    .issuance_date("2022-07-01T00:00:00Z")
    .credential_schema(
        vec![CredentialSchemaBuilder::default()
        .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_achievementcredential_schema.json")
        .type_("1EdTechJsonSchemaValidator2019")
        .try_into()
        .unwrap()])
    .try_into()
    .unwrap();

    // Here we test the built struct against the struct deserialized from the example .json file.

    let file =
        File::open("tests/obv3_json_examples/skill_assertion_credential_engine.json").expect("Failed to open file");
    let skill_assertion_cred_engine_from_file: AchievementCredential =
        serde_json::from_reader(&file).expect("Couldn't read from file");

    assert_eq!(skill_assertion_credential_engine, skill_assertion_cred_engine_from_file);

    // Here we test the built struct converted to a json_value against the json_value deserialized from the example .json file

    let file =
        File::open("tests/obv3_json_examples/skill_assertion_credential_engine.json").expect("Failed to open file");
    let json_value_from_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(
        serde_json::to_value(skill_assertion_credential_engine).unwrap(),
        json_value_from_file
    );
}
