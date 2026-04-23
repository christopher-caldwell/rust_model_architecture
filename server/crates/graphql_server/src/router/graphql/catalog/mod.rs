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
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
    author_name: String,
}

impl From<Book> for CatalogTitle {
    fn from(value: Book) -> Self {
        Self {
            isbn: value.isbn,
            created_at: value.dt_created,
            updated_at: value.dt_modified,
            title: value.title,
            author_name: value.author_name,
        }
    }
}

#[derive(SimpleObject)]
pub struct InventoryCopy {
    barcode: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    author_name: String,
    status: InventoryCopyStatus,
}

impl From<BookCopy> for InventoryCopy {
    fn from(value: BookCopy) -> Self {
        Self {
            barcode: value.barcode,
            created_at: value.dt_created,
            updated_at: value.dt_modified,
            author_name: value.author_name,
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
    author_name: String,
}
