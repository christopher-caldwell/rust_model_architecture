use anyhow::Context;
use domain::{book::Book, book_copy::BookCopy};
use std::sync::Arc;

use crate::ports::read_repos::CatalogReadRepoPort;

#[derive(Clone)]
pub struct CatalogQueries {
    catalog_read_repo: Arc<dyn CatalogReadRepoPort>,
}

impl CatalogQueries {
    #[must_use]
    pub fn new(
        catalog_read_repo: Arc<dyn CatalogReadRepoPort>,
    ) -> Self {
        Self { catalog_read_repo }
    }

    pub async fn get_book_catalog(
        &self,
    ) -> anyhow::Result<Vec<Book>> {
        let result = self
            .catalog_read_repo
            .get_book_catalog()
            .await
            .context("Failed to get book catalog")?;

        Ok(result)
    }

    pub async fn get_book_copy_details(
        &self,
        book_copy_id: i64,
    ) -> anyhow::Result<Option<BookCopy>> {
        let result = self
            .catalog_read_repo
            .get_book_copy_details(book_copy_id)
            .await
            .context("Failed to get book copy details")?;

        Ok(result)
    }
}
