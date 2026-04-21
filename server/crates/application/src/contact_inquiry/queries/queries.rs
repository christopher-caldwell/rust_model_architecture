use domain::contact_inquiry::ContactInquiry;
use std::sync::Arc;

use crate::contact_inquiry::{ContactInquiryReadRepoPort, ContactInquiryStatusRepoPort};

#[derive(Clone)]
pub struct ContactInquiryQueries {
    contact_inquiry_read: Arc<dyn ContactInquiryReadRepoPort>,
    _contact_inquiry_status: Arc<dyn ContactInquiryStatusRepoPort>,
}

impl ContactInquiryQueries {
    #[must_use]
    pub fn new(
        contact_inquiry_read: Arc<dyn ContactInquiryReadRepoPort>,
        contact_inquiry_status: Arc<dyn ContactInquiryStatusRepoPort>,
    ) -> Self {
        Self {
            contact_inquiry_read,
            _contact_inquiry_status: contact_inquiry_status,
        }
    }

    /// # Errors
    ///
    /// Returns an error if the underlying function returns one
    pub async fn get_by_ident(&self, ident: &str) -> Result<Option<ContactInquiry>, String> {
        self.contact_inquiry_read.get_by_ident(ident).await
    }

    /// # Errors
    ///
    /// Returns an error if the underlying function returns one
    pub async fn get_contact_inquires(&self) -> Result<Vec<ContactInquiry>, String> {
        self.contact_inquiry_read.get_contact_inquiries().await
    }
}
