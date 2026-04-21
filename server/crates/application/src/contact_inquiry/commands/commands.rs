use std::sync::Arc;

use anyhow::Context;
use domain::contact_inquiry::{ContactInquiry, ContactInquiryCreationPayload};

use crate::{
    contact_inquiry::commands::ContactInquirySpamRatingPort, gen_ident::gen_ident,
    uow_port::WriteUnitOfWorkFactory,
};

#[derive(Clone)]
pub struct ContactInquiryCommands {
    uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
    spam_service: Arc<dyn ContactInquirySpamRatingPort>,
}

impl ContactInquiryCommands {
    #[must_use]
    pub fn new(
        uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
        spam_service: Arc<dyn ContactInquirySpamRatingPort>,
    ) -> Self {
        Self {
            uow_factory,
            spam_service,
        }
    }

    /// # Errors
    ///
    /// Returns `anyhow::Error` if the spam rating cannot be determined.
    /// Returns `anyhow::Error` if the contact inquiry cannot be created.
    /// Returns `anyhow::Error` if the transaction fails.
    pub async fn create(
        &self,
        payload: ContactInquiryCreationPayload,
    ) -> Result<ContactInquiry, anyhow::Error> {
        // Non-DB work first — avoid holding a transaction connection during the LLM call
        let spam_rating = self
            .spam_service
            .get_spam_likelihood(&payload.message)
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .context("Error getting spam rating")?;
        let ident = gen_ident();
        let prepared = payload.prepare(ident, spam_rating);

        // Open transactional UoW only for the DB operations
        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .contact_inquiry_write_repo()
            .create_contact_inquiry(&prepared)
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .context("Failed to insert contact inquiry")?;
        uow.commit()
            .await
            .context("Failed to commit transaction")?;

        Ok(result)
    }
}
