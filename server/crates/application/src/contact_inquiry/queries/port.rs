use async_trait::async_trait;
use domain::contact_inquiry::ContactInquiry;
use std::collections::HashSet;

#[async_trait]
pub trait ContactInquiryReadRepoPort: Send + Sync {
    async fn get_contact_inquiry_by_id(&self, id: i16) -> Result<Option<ContactInquiry>, String>;
    async fn get_by_ident(&self, ident: &str) -> Result<Option<ContactInquiry>, String>;
    async fn get_contact_inquiries(&self) -> Result<Vec<ContactInquiry>, String>;
}

#[async_trait]
pub trait ContactInquiryStatusRepoPort: Send + Sync {
    async fn get_valid_statuses(&self) -> Result<HashSet<String>, String>;
    async fn get_status_ident_by_id(&self, id: &i16) -> Result<String, String>;
    async fn get_status_id_by_ident(&self, ident: &str) -> Result<i16, String>;
}
