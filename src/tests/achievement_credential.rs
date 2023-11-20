use crate::json_schema::achievement_credential::AchievementCredential;

fn achievement_credential_assert_eq( A: AchievementCredential, B: AchievementCredential) {
    assert_eq!(A.awarded_date, B.awarded_date);
    assert_eq!(A.context, B.context);
    assert_eq!(A.credential_schema, B.credential_schema);
    assert_eq!(A.credential_status, B.credential_status);
    assert_eq!(A.credential_subject, B.credential_subject);
    //assert_eq!(A., B.);
}

#[cfg(test)]
#[test]
fn test_achievement_credential() {
    use crate::{json_schema::{achievement::{Achievement, Criteria}, achievement_credential::AchievementCredential, general::Context, achievement_subject::AchievementSubject}, tests::json_example};

    let json_achievement_credential: AchievementCredential = json_example("json_examples/achievement_credential");

    let _achievement_credential: AchievementCredential = AchievementCredential::builder()
        .context(vec![Context::SingleString(
            "https://www.w3.org/2018/credentials/examples/v1".to_string(),
        )])
        .credential_subject(
            AchievementSubject::builder()
                .achievement(Achievement::builder().criteria(Criteria::builder())),
        )
        .try_into()
        .unwrap();

     achievement_credential_assert_eq(json_achievement_credential, _achievement_credential);
}
