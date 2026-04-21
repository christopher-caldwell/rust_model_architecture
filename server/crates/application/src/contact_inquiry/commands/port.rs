use async_trait::async_trait;
use domain::contact_inquiry::{ContactInquiry, ContactInquiryPrepared};

#[async_trait]
pub trait ContactInquiryWriteRepoPort: Send + Sync {
    async fn create_contact_inquiry(
        &self,
        insert: &ContactInquiryPrepared,
    ) -> Result<ContactInquiry, String>;
}

#[async_trait]
pub trait ContactInquirySpamRatingPort: Send + Sync {
    async fn get_spam_likelihood(&self, message: &str) -> Result<u8, String>;
}
