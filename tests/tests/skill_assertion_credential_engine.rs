use crate::tests::assert_eq_json_value;
use openbadges::{
    achievement::{Achievement, AchievementBuilder, Criteria, CriteriaBuilder, AchievementType},
    achievement_credential::{AchievementCredential, AchievementCredentialBuilder, AchievementCredentialType, CredentialSchema, CredentialSchemaBuilder, AchievementCredentialSchema},
    achievement_subject::{AchievementSubject, AchievementSubjectBuilder},
    profile::{Profile, ProfileBuilder}, alignment::{Alignment, AlignmentBuilder, AlignmentTargetType}, general::{Image, ImageBuilder},
};
use std::{fs::File, str::FromStr};

#[test]
fn skill_assertion_credential_engine() {

    // Testing if serialization and deserialization between the OBv3 examples and our rust code works as needed.

    assert_eq_json_value::<AchievementCredential>("tests/obv3_json_examples/skill_assertion_credential_engine.json");

    // Next, the builders are tested against the OBv3 examples 

    let skill_assertion_credential_engine: AchievementCredential = AchievementCredentialBuilder::default()
    .context(vec![
        "https://www.w3.org/2018/credentials/v1".into(),
		"https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
		"https://purl.imsglobal.org/spec/ob/v3p0/extensions.json".into()
    ])
    .id("http://1edtech.edu/credentials/3732")
    .type_(AchievementCredentialType::from(vec!["VerifiableCredential", "OpenBadgeCredential"]))
    .name("Solve and graph linear equations and inequalities")
    .credential_subject({
        let credential_subject: AchievementSubject = AchievementSubjectBuilder::default()
        .id("did:example:ebfeb1f712ebc6f1c276e12ec21".to_string())
        .type_("AchievementSubject",)
        .achievement({
            let achievement: Achievement = AchievementBuilder::default()
            .id("https://example.com/achievements/math/linear-1")
            .type_("Achievement")
            .alignment({
                let alignment: Alignment = AlignmentBuilder::default()
                .type_("Alignment")
                .target_code("ce-6369c51f-4d86-4592-a761-8b32ae70a045".to_string())
                .target_framework("Ivy Tech Community College of Indiana, MATH 135, FINITE MATH".to_string())
                .target_name("Solve and graph linear equations and inequalities")
                .target_type(AlignmentTargetType::from_str("ceasn:Competency").unwrap())
                .target_url("https://credentialfinder.org/competency/ce-6369c51f-4d86-4592-a761-8b32ae70a045")
                .try_into()
                .unwrap();
    
                vec![alignment]
            })
            .achievement_type(AchievementType::from_str("Competency").unwrap())
            .creator({
                let creator: Profile = ProfileBuilder::default()
                .id("https://example.com/issuers/123767")
                .type_("Profile")
                .name("Example Industry Group".to_string())
                .url("https://example.com".to_string())
                .description("Example Industry Group is a consortium of luminaries who publish skills data for common usage.".to_string())
                .email("info@exammple.com".to_string())
                .try_into()
                .unwrap();

                creator
            })
            .criteria({
                let criteria: Criteria = CriteriaBuilder::default()
                .narrative("Learners must demonstrate understanding of linear algebra and graphic representation of linear equations.".to_string())
                .try_into()
                .unwrap();
                
                criteria
            })
            .description("This achievement represents developing capability to solve and graph linear equations and inequalities")
            .image({
                let image: Image = ImageBuilder::default()
                .id("https://example.com/achievements/math/linear-1/image")
                .type_("Image")
                .caption("A line, sloping upward optimistically".to_string())
                .try_into()
                .unwrap();

                image
            })
            .name("Linear equations and inequalities")
            .try_into()
            .unwrap();
    
            achievement
        })
        .try_into()
        .unwrap();
    
        credential_subject
    })
    .issuer({
        let issuer: Profile = ProfileBuilder::default()
        .id("https://1edtech.edu/issuers/565049")
        .type_("Profile")
        .name("1EdTech University".to_string())
        .url("https://1edtech.edu".to_string())
        .phone("1-222-333-4444".to_string())
        .description("1EdTech University provides online degree programs.".to_string())
        .image({
            let image: Image = ImageBuilder::default()
            .id("https://1edtech.edu/logo.png")
            .type_("Image")
            .caption("1EdTech University logo".to_string())
            .try_into()
            .unwrap();

            image
        })
        .email("registrar@1edtech.edu".to_string())
        .try_into()
        .unwrap();

        issuer
    })
    .issuance_date("2022-07-01T00:00:00Z".parse::<chrono::DateTime<chrono::offset::Utc>>().unwrap())
    .credential_schema({
        let schema: CredentialSchema = CredentialSchemaBuilder::default()
        .id("https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_achievementcredential_schema.json")
        .type_("1EdTechJsonSchemaValidator2019")
        .try_into()
        .unwrap();

        AchievementCredentialSchema::from(vec![schema])
    })
    .try_into()
    .unwrap();

    // Here we test the built struct against the struct deserialized from the example .json file.
    
    let file = File::open("tests/obv3_json_examples/skill_assertion_credential_engine.json").expect("Failed to open file");
    let skill_assertion_cred_engine_from_file: AchievementCredential = serde_json::from_reader(&file).expect("Couldn't read from file");
    
    assert_eq!(skill_assertion_credential_engine, skill_assertion_cred_engine_from_file);
    
    // Here we test the built struct converted to a json_value against the json_value deserialized from the example .json file

    let file = File::open("tests/obv3_json_examples/skill_assertion_credential_engine.json").expect("Failed to open file");
    let json_value_from_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(serde_json::to_value(skill_assertion_credential_engine).unwrap(), json_value_from_file);

}
