use anyhow::Context;
use domain::{
    book::{port::BookReadRepoPort, Book},
    book_copy::{port::BookCopyReadRepoPort, BookCopy},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct CatalogQueries {
    book_read_repo: Arc<dyn BookReadRepoPort>,
    book_copy_read_repo: Arc<dyn BookCopyReadRepoPort>,
}

impl CatalogQueries {
    #[must_use]
    pub fn new(
        book_read_repo: Arc<dyn BookReadRepoPort>,
        book_copy_read_repo: Arc<dyn BookCopyReadRepoPort>,
    ) -> Self {
        Self {
            book_read_repo,
            book_copy_read_repo,
        }
    }

    pub async fn get_book_catalog(&self) -> anyhow::Result<Vec<Book>> {
        self.book_read_repo
            .get_catalog()
            .await
            .context("Failed to get book catalog")
    }

    pub async fn get_book_by_isbn(&self, isbn: &str) -> anyhow::Result<Option<Book>> {
        self.book_read_repo
            .get_by_isbn(isbn)
            .await
            .context("Failed to get book by isbn")
    }

    pub async fn get_book_copy_details(&self, barcode: &str) -> anyhow::Result<Option<BookCopy>> {
        self.book_copy_read_repo
            .get_by_barcode(barcode)
            .await
            .context("Failed to get book copy details")
    }
}
