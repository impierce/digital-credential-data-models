use crate::tests::assert_eq_json_value;
use openbadges::json_schema::{
    achievement::{Achievement, AchievementBuilder, Criteria, CriteriaBuilder, AchievementType},
    achievement_credential::{AchievementCredential, AchievementCredentialBuilder, AchievementCredentialType},
    achievement_subject::{AchievementSubject, AchievementSubjectBuilder},
    profile::{Profile, ProfileBuilder}, general::{ImageBuilder, Image}, identity::{IdentityObject, IdentityObjectBuilder}, alignment::{Alignment, AlignmentBuilder, AlignmentTargetType},
};
use std::fs::File;

#[test]
fn full_achievement_credential() {

    // Testing if serialization and deserialization between the OBv3 examples and our rust code works as needed.

    assert_eq_json_value::<AchievementCredential>("tests/obv3_json_examples/full_achievement_credential.json");
    
    // Next, the builders are tested against the OBv3 examples 

    let full_achievement_credential: AchievementCredential = AchievementCredentialBuilder::default()
    .context([
        "https://www.w3.org/2018/credentials/v1".into(),
        "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.2.json".into(),
        "https://purl.imsglobal.org/spec/ob/v3p0/extensions.json".into()
    ])
    .id("http://1edtech.edu/credentials/3732")
    .type_(AchievementCredentialType::from(vec![
        "VerifiableCredential",
        "OpenBadgeCredential"
      ]))
    .name("1EdTech University Degree for Example Student")
    .description("1EdTech University Degree Description".to_string())
    .image({
        let image: Image = ImageBuilder::default()
        .id("https://1edtech.edu/credentials/3732/image")
        .type_("Image")
        .caption("1EdTech University Degree for Example Student".to_string())
        .try_into()
        .unwrap();

        image
    })
    .credential_subject({
        let credential_subject: AchievementSubject = AchievementSubjectBuilder::default()
        .id("did:example:ebfeb1f712ebc6f1c276e12ec21".to_string())
        .type_("AchievementSubject")
        .activity_end_date("2010-01-02T00:00:00Z"
            .parse::<chrono::DateTime<chrono::offset::Utc>>()
            .unwrap()
        )
        .activity_start_date("2010-01-01T00:00:00Z"
            .parse::<chrono::DateTime<chrono::offset::Utc>>()
            .unwrap()        
        )
        .credits_earned(42.0)
        .license_number("A-9320041".to_string())
        .role("Major Domo".to_string())
        .source({
            let source: Profile = ProfileBuilder::default()
            .id("https://school.edu/issuers/201234".to_string())
            .type_("Profile")
            .name("1EdTech College of Arts".to_string())
            .try_into()
            .unwrap();

            source
        })
        .term("Fall".to_string())
        .identifier({
            let identifier_student: IdentityObject = IdentityObjectBuilder::default()
            .type_("IdentityObject")
            .identity_hash("student@1edtech.edu")
            .identity_type("emailAddress")
            .hashed(false)
            .salt("not-used".to_string())
            .try_into()
            .unwrap();

            let identifier_somebody: IdentityObject = IdentityObjectBuilder::default()
            .type_("IdentityObject")
            .identity_hash("somebody@gmail.com")
            .identity_type("emailAddress")
            .hashed(false)
            .salt("not-used".to_string())            
            .try_into()
            .unwrap();

            vec![identifier_student, identifier_somebody]
        })
        .achievement({
            let achievement: Achievement = AchievementBuilder::default()
            .id("https://1edtech.edu/achievements/degree")
            .type_("Achievement")
            .alignment({
                let alignment_cfitem: Alignment = AlignmentBuilder::default()
                .type_("Alignment")
                .target_code("degree".to_string())
                .target_description("1EdTech University Degree programs.".to_string())
                .target_name("1EdTech University Degree")
                .target_framework("1EdTech University Program and Course Catalog".to_string())
                .target_type(AlignmentTargetType::from("CFItem"))
                .target_url("https://1edtech.edu/catalog/degree")
                .try_into()
                .unwrap();

                let alignment_ctdl: Alignment = AlignmentBuilder::default()
                .type_("Alignment")
                .target_code("degree".to_string())
                .target_description("1EdTech University Degree programs.".to_string())
                .target_name("1EdTech University Degree")
                .target_framework("1EdTech University Program and Course Catalog".to_string())
                .target_type(AlignmentTargetType::from("CTDL"))
                .target_url("https://1edtech.edu/catalog/degree")
                .try_into()
                .unwrap();

                vec![alignment_cfitem, alignment_ctdl]
            })
            .achievement_type(AchievementType::from("Degree"))
            .creator({
                let creator: Profile = ProfileBuilder::default()
                .id("https://1edtech.edu/issuers/565049")
                .type_("Profile")
                .name("1EdTech University".to_string())
                .url("https://1edtech.edu".to_string())
                .phone("1-222-333-4444".to_string())
                .description("1EdTech University provides online degree programs.".to_string())
                .endorsement({
                    let endorsement:
                })
                .try_into()
                .unwrap();

                creator
            })
            .try_into()
            .unwrap();
            achievement
        })
        .try_into()
        .unwrap();
        credential_subject
    })   
    .try_into()
    .unwrap();

    let file = File::open("tests/obv3_json_examples/basic_achievement_credential.json").expect("Failed to open file");
    let json_value_from_file: serde_json::Value = serde_json::from_reader(file).expect("Couldn't read from file");

    assert_eq!(serde_json::to_value(full_achievement_credential).unwrap(), json_value_from_file)
}
