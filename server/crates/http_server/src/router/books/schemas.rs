use chrono::{DateTime, Utc};
use domain::book::{Book, BookCreationPayload};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const BOOKS_TAG: &str = "Books";
pub const BOOKS_PATH: &str = "/books";
pub const BOOK_COPIES_BY_BOOK_ID_PATH: &str = "/books/{book_id}/copies";

#[derive(Serialize, ToSchema)]
pub struct BookResponseBody {
    pub id: i16,
    pub isbn: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub title: String,
    pub author_name: String,
}

impl From<Book> for BookResponseBody {
    fn from(value: Book) -> Self {
        Self {
            id: value.id.0,
            isbn: value.isbn,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            title: value.title,
            author_name: value.author_name,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateBookRequestBody {
    pub isbn: String,
    pub title: String,
    pub author_name: String,
}

impl From<CreateBookRequestBody> for BookCreationPayload {
    fn from(value: CreateBookRequestBody) -> Self {
        Self {
            isbn: value.isbn,
            title: value.title,
            author_name: value.author_name,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateBookCopyRequestBody {
    pub barcode: String,
    pub author_name: String,
}
