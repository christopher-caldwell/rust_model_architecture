use application::contact_inquiry::ContactInquiryReadRepoPort;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::contact_inquiry::ContactInquiry;
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct ContactInquiryDbRow {
    pub contact_inquiry_id: i16,
    pub contact_inquiry_ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub first_name: String,
    pub last_name: String,
    pub website_given: String,
    pub email: String,
    pub phone_number: String,
    pub source: String,
    pub message: String,
    pub status: String,
    pub spam_likelihood: i16,
}

impl From<ContactInquiryDbRow> for ContactInquiry {
    fn from(value: ContactInquiryDbRow) -> Self {
        ContactInquiry {
            id: value.contact_inquiry_id,
            ident: value.contact_inquiry_ident,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            status: value.status,
            first_name: value.first_name,
            last_name: value.last_name,
            website_given: value.website_given,
            email: value.email,
            phone_number: value.phone_number,
            source: value.source,
            message: value.message,
            spam_likelihood: value.spam_likelihood,
        }
    }
}

pub struct ContactInquiryReadRepoSql {
    pub pool: PgPool,
}

#[async_trait]
impl ContactInquiryReadRepoPort for ContactInquiryReadRepoSql {
    async fn get_contact_inquiry_by_id(&self, id: i16) -> Result<Option<ContactInquiry>, String> {
        let row = sqlx::query_file_as!(
            ContactInquiryDbRow,
            "sql/contact_inquiry/queries/get_by_id.sql",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(ContactInquiryDbRow::into))
    }

    async fn get_by_ident(&self, ident: &str) -> Result<Option<ContactInquiry>, String> {
        let row = sqlx::query_file_as!(
            ContactInquiryDbRow,
            "sql/contact_inquiry/queries/get_by_ident.sql",
            ident
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(ContactInquiryDbRow::into))
    }

    async fn get_contact_inquiries(&self) -> Result<Vec<ContactInquiry>, String> {
        let rows = sqlx::query_file_as!(
            ContactInquiryDbRow,
            "sql/contact_inquiry/queries/get_list.sql"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        /*
         * rows` is a `Vec<ContactInquiryDbRow>`, so `.into_iter()` gives owned values, `.map(Into::into)` calls your `From` impl on each, and
         * `.collect()` gathers them into a `Vec<ContactInquiry>`.
         */
        Ok(rows.into_iter().map(Into::into).collect())
    }
}
