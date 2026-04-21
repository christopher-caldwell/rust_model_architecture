use application::contact_inquiry::ContactInquiryStatusRepoPort;
use async_trait::async_trait;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

#[derive(sqlx::FromRow)]
pub struct ContactInquiryStatus {
    pub id: i16,
    pub status: String,
}

pub struct ContactInquiryStatusRepoSql {
    pub id_cache: RwLock<HashMap<i16, String>>,
    pub ident_cache: RwLock<HashMap<String, i16>>,
    pub pool: PgPool,
}

#[async_trait]
impl ContactInquiryStatusRepoPort for ContactInquiryStatusRepoSql {
    async fn get_valid_statuses(&self) -> Result<HashSet<String>, String> {
        let rows = sqlx::query_file_as!(
            ContactInquiryStatus, // Pass the single-row type
            "sql/contact_inquiry/queries/get_statuses.sql",
        )
        .fetch_all(&self.pool) // This is what gives you the Vec<ContactInquiryStatus>
        .await
        .map_err(|e| e.to_string())?;

        {
            let mut id_cache = self.id_cache.write().await;
            let mut ident_cache = self.ident_cache.write().await;
            for row in &rows {
                id_cache.insert(row.id, row.status.clone());
                ident_cache.insert(row.status.clone(), row.id);
            }
        }

        // Map the rows into the HashSet your function signature returns
        let statuses: HashSet<String> = rows.into_iter().map(|row| row.status).collect();
        Ok(statuses)
    }
    async fn get_status_ident_by_id(&self, id: &i16) -> Result<String, String> {
        {
            let cache = self.id_cache.read().await;
            if let Some(ident) = cache.get(id) {
                return Ok(ident.clone());
            }
        }

        /*
         * Future work:
         * Change the return value to 2 hash maps, one by i16 ID and the other by String ident
         * Depending on the caller, will use O(1) lookup instead of re-acquiring the cache.
         */
        let _ = self.get_valid_statuses().await?;

        let cache = self.id_cache.read().await;
        match cache.get(id) {
            Some(ident) => Ok(ident.clone()),
            None => Err(format!("Status ID '{id}' does not exist")),
        }
    }
    async fn get_status_id_by_ident(&self, ident: &str) -> Result<i16, String> {
        {
            let cache = self.ident_cache.read().await;
            if let Some(id) = cache.get(ident) {
                return Ok(*id);
            }
        }

        /*
         * Future work:
         * Change the return value to 2 hash maps, one by i16 ID and the other by String ident
         * Depending on the caller, will use O(1) lookup instead of re-acquiring the cache.
         */
        let _ = self.get_valid_statuses().await?;

        let cache = self.ident_cache.read().await;
        match cache.get(ident) {
            Some(id) => Ok(*id),
            None => Err(format!("Status ident '{ident}' does not exist")),
        }
    }
}
