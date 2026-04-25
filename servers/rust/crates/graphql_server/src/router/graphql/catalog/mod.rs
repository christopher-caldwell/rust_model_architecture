use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use domain::{
    book::{Book, BookCreationPayload},
    book_copy::{BookCopy, BookCopyStatus},
};

pub mod mutations;
pub mod queries;

pub use mutations::CatalogMutation;
pub use queries::CatalogQuery;

#[derive(SimpleObject)]
pub struct CatalogTitle {
    isbn: String,
    dt_created: DateTime<Utc>,
    dt_modified: DateTime<Utc>,
    title: String,
    author_name: String,
}

impl From<Book> for CatalogTitle {
    fn from(value: Book) -> Self {
        Self {
            isbn: value.isbn,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            title: value.title,
            author_name: value.author_name,
        }
    }
}

#[derive(SimpleObject)]
pub struct InventoryCopy {
    barcode: String,
    dt_created: DateTime<Utc>,
    dt_modified: DateTime<Utc>,
    status: InventoryCopyStatus,
}

impl From<BookCopy> for InventoryCopy {
    fn from(value: BookCopy) -> Self {
        Self {
            barcode: value.barcode,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            status: InventoryCopyStatus::from(value.status),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum InventoryCopyStatus {
    Active,
    Maintenance,
    Lost,
}

impl From<BookCopyStatus> for InventoryCopyStatus {
    fn from(value: BookCopyStatus) -> Self {
        match value {
            BookCopyStatus::Active => Self::Active,
            BookCopyStatus::Maintenance => Self::Maintenance,
            BookCopyStatus::Lost => Self::Lost,
        }
    }
}

#[derive(InputObject)]
pub struct CreateCatalogTitleInput {
    isbn: String,
    title: String,
    author_name: String,
}

impl From<CreateCatalogTitleInput> for BookCreationPayload {
    fn from(value: CreateCatalogTitleInput) -> Self {
        Self {
            isbn: value.isbn,
            title: value.title,
            author_name: value.author_name,
        }
    }
}

#[derive(InputObject)]
pub struct AddInventoryCopyInput {
    isbn: String,
    barcode: String,
}
