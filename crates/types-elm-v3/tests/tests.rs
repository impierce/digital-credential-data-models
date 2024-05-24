#[cfg(test)]
mod tests {
    use types_elm_v3::*;

    #[test]
    fn test_email() -> serde_json::Result<()> {
        let value = serde_json::json!({
            "type": "Mailbox",
            "id": "valid@email.com"
        });

        let mailbox: Mailbox = serde_json::from_value(value)?;

        if let Some(Email::EmailAddress(addr)) = mailbox.id {
            assert_eq!(addr.as_str(), "valid@email.com");
            Ok(())
        } else {
            panic!("Should be an email address");
        }
    }

    #[test]
    fn test_invalid_email() {
        let value = serde_json::json!({
            "type": "Mailbox",
            "id": "not-valid.email.com"
        });

        let mailbox: serde_json::Result<Mailbox> = serde_json::from_value(value);
        assert!(mailbox.is_err());
    }

    #[test]
    fn test_mail_to() -> serde_json::Result<()> {
        let value = serde_json::json!({
            "type": "Mailbox",
            "id": "mailto:valid@email.com"
        });

        let mailbox: Mailbox = serde_json::from_value(value)?;

        if let Some(Email::MailTo(mail_to)) = mailbox.id {
            assert_eq!(mail_to.as_str(), "mailto:valid@email.com");
        }

        Ok(())
    }

    #[test]
    fn test_invalid_mail_to()  {
        let value = serde_json::json!({
            "type": "Mailbox",
            "id": "mailto:invalid.email.com"
        });

        let mailbox: serde_json::Result<Mailbox> = serde_json::from_value(value);
        assert!(mailbox.is_err());
    }
}
