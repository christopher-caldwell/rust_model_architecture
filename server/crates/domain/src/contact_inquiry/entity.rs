use chrono::{DateTime, Utc};

const SPAM_LIKELIHOOD_THRESHOLD: u8 = 30;

pub struct ContactInquiry {
    pub id: i16,
    pub ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: String,
    pub spam_likelihood: i16,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub source: String,
    pub website_given: String,
    pub message: String,
}

pub struct ContactInquiryCreationPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub source: String,
    pub website_given: String,
    pub message: String,
}

pub struct ContactInquiryPrepared {
    pub ident: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub source: String,
    pub website_given: String,
    pub message: String,
    pub status: &'static str,
    pub spam_likelihood: i16,
}

impl ContactInquiryCreationPayload {
    #[must_use]
    pub fn is_spam(&self, likelihood: u8) -> bool {
        likelihood >= SPAM_LIKELIHOOD_THRESHOLD
    }

    #[must_use]
    pub fn prepare(self, ident: String, spam_rating: u8) -> ContactInquiryPrepared {
        let is_spam = self.is_spam(spam_rating);
        let status: &str = if is_spam { "spam" } else { "unread" };
        ContactInquiryPrepared {
            ident,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            phone_number: self.phone_number,
            source: self.source,
            website_given: self.website_given,
            message: self.message,
            status,
            spam_likelihood: i16::from(spam_rating),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_payload() -> ContactInquiryCreationPayload {
        ContactInquiryCreationPayload {
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            phone_number: String::new(),
            source: String::new(),
            website_given: String::new(),
            message: String::new(),
        }
    }

    #[test]
    fn is_not_spam_when_likelihood_below_threshold() {
        let payload = make_payload();
        assert!(!payload.is_spam(0));
        assert!(!payload.is_spam(15));
        assert!(!payload.is_spam(29));
    }

    #[test]
    fn is_spam_when_likelihood_at_threshold() {
        let payload = make_payload();
        assert!(payload.is_spam(30));
    }

    #[test]
    fn is_spam_when_likelihood_above_threshold() {
        let payload = make_payload();
        assert!(payload.is_spam(50));
        assert!(payload.is_spam(100));
    }

    #[test]
    fn prepares_correctly_when_likelihood_above_threshold() {
        let ident = String::from("123");
        let payload_30 = make_payload();
        let payload_50 = make_payload();
        let payload_75 = make_payload();
        let payload_100 = make_payload();
        let prepared_30 = payload_30.prepare(ident.clone(), 30);
        let prepared_50 = payload_50.prepare(ident.clone(), 50);
        let prepared_75 = payload_75.prepare(ident.clone(), 75);
        let prepared_100 = payload_100.prepare(ident.clone(), 100);
        let spam_status = String::from("spam");
        assert_eq!(prepared_30.status.to_string(), spam_status);
        assert_eq!(prepared_50.status.to_string(), spam_status);
        assert_eq!(prepared_75.status.to_string(), spam_status);
        assert_eq!(prepared_100.status.to_string(), spam_status);
    }

    #[test]
    fn prepares_correctly_when_likelihood_below_threshold() {
        let ident = String::from("123");
        let payload_00 = make_payload();
        let payload_05 = make_payload();
        let payload_10 = make_payload();
        let payload_20 = make_payload();
        let payload_29 = make_payload();
        let prepared_00 = payload_00.prepare(ident.clone(), 0);
        let prepared_05 = payload_05.prepare(ident.clone(), 5);
        let prepared_10 = payload_10.prepare(ident.clone(), 10);
        let prepared_20 = payload_20.prepare(ident.clone(), 20);
        let prepared_29 = payload_29.prepare(ident.clone(), 29);
        let unread_status = String::from("unread");
        assert_eq!(prepared_00.status.to_string(), unread_status);
        assert_eq!(prepared_05.status.to_string(), unread_status);
        assert_eq!(prepared_10.status.to_string(), unread_status);
        assert_eq!(prepared_20.status.to_string(), unread_status);
        assert_eq!(prepared_29.status.to_string(), unread_status);
    }
}
