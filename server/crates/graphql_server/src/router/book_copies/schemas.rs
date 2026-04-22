use chrono::{DateTime, Utc};
use domain::book_copy::{BookCopy, BookCopyStatus};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const BOOK_COPIES_TAG: &str = "Book Copies";
pub const BOOK_COPIES_PATH: &str = "/book-copies";
pub const BOOK_COPY_BY_ID_PATH: &str = "/book-copies/{barcode}";
pub const BOOK_COPY_LOSS_PATH: &str = "/book-copies/{barcode}/loss";
pub const BOOK_COPY_MAINTENANCE_PATH: &str = "/book-copies/{barcode}/maintenance";
pub const BOOK_COPY_RETURNS_PATH: &str = "/book-copies/{barcode}/returns";
pub const BOOK_COPY_LOSS_REPORTS_PATH: &str = "/book-copies/{barcode}/loss-reports";

#[derive(Serialize, ToSchema)]
pub struct BookCopyResponseBody {
    pub barcode: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub author_name: String,
    pub status: String,
}

impl From<BookCopy> for BookCopyResponseBody {
    fn from(value: BookCopy) -> Self {
        Self {
            barcode: value.barcode,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            author_name: value.author_name,
            status: book_copy_status_text(&value.status),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateBookCopyRequestBody {
    pub isbn: String,
    pub barcode: String,
    pub author_name: String,
}

fn book_copy_status_text(status: &BookCopyStatus) -> String {
    match status {
        BookCopyStatus::Active => String::from("active"),
        BookCopyStatus::Maintenance => String::from("maintenance"),
        BookCopyStatus::Lost => String::from("lost"),
    }
}
