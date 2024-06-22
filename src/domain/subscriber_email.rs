use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        if ValidateEmail::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{s} is not a valid subscriber name"))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};
    use fake::{faker::internet::en::SafeEmail, Fake};

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_owned();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_owned();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_owned();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }
}
