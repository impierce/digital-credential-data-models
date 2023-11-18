pub mod json_schema;

#[cfg(test)]
mod tests {
    use crate::json_schema::achievement::{Achievement, Criteria};
    use crate::json_schema::achievement_credential::AchievementCredential;
    use crate::json_schema::achievement_subject::AchievementSubject;
    use crate::json_schema::general::Context;

    #[test]
    fn test_achievement_credential() {
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
    }
}
