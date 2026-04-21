use std::sync::Arc;

use application::contact_inquiry::ContactInquiryWriteRepoPort;
use async_trait::async_trait;
use chrono::Utc;
use domain::contact_inquiry::{ContactInquiry, ContactInquiryPrepared};
use sqlx::{Postgres, Transaction};
use tokio::sync::Mutex;

#[derive(sqlx::FromRow)]
struct ContactInquiryPreparedResult {
    contact_inquiry_id: i16,
}

pub struct ContactInquiryWriteRepoTx {
    pub tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

#[async_trait]
impl ContactInquiryWriteRepoPort for ContactInquiryWriteRepoTx {
    async fn create_contact_inquiry(
        &self,
        insert: &ContactInquiryPrepared,
    ) -> Result<ContactInquiry, String> {
        let mut guard = self.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or_else(|| "Transaction already consumed".to_string())?;
        let prepared_result = sqlx::query_file_as!(
            ContactInquiryPreparedResult,
            "sql/contact_inquiry/commands/create.sql",
            insert.ident,
            insert.status.to_string(),
            insert.first_name,
            insert.last_name,
            insert.email,
            insert.phone_number,
            insert.source,
            insert.website_given,
            insert.message,
            insert.spam_likelihood,
        )
        .fetch_one(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(ContactInquiry {
            id: prepared_result.contact_inquiry_id,
            ident: insert.ident.clone(),
            dt_created: Utc::now(),
            dt_modified: Utc::now(),
            status: insert.status.to_string(),
            first_name: insert.first_name.clone(),
            last_name: insert.last_name.clone(),
            website_given: insert.website_given.clone(),
            email: insert.email.clone(),
            phone_number: insert.phone_number.clone(),
            source: insert.source.clone(),
            message: insert.message.clone(),
            spam_likelihood: insert.spam_likelihood,
        })
    }
}
