use chrono::{DateTime, Utc};
use domain::book_copy::{BookCopy, BookCopyStatus};
use serde::Serialize;
use utoipa::ToSchema;

pub const BOOK_COPIES_TAG: &str = "Book Copies";
pub const BOOK_COPIES_PATH: &str = "/book-copies";
pub const BOOK_COPY_BY_ID_PATH: &str = "/book-copies/{id}";
pub const BOOK_COPY_LOSS_PATH: &str = "/book-copies/{id}/loss";
pub const BOOK_COPY_MAINTENANCE_PATH: &str = "/book-copies/{id}/maintenance";
pub const BOOK_COPY_RETURNS_PATH: &str = "/book-copies/{id}/returns";
pub const BOOK_COPY_LOSS_REPORTS_PATH: &str = "/book-copies/{id}/loss-reports";

#[derive(Serialize, ToSchema)]
pub struct BookCopyResponseBody {
    pub id: i64,
    pub barcode: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_id: i16,
    pub author_name: String,
    pub status: String,
}

impl From<BookCopy> for BookCopyResponseBody {
    fn from(value: BookCopy) -> Self {
        Self {
            id: value.id.0,
            barcode: value.barcode,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            book_id: value.book_id.0,
            author_name: value.author_name,
            status: book_copy_status_text(&value.status),
        }
    }
}

fn book_copy_status_text(status: &BookCopyStatus) -> String {
    match status {
        BookCopyStatus::Active => String::from("active"),
        BookCopyStatus::Maintenance => String::from("maintenance"),
        BookCopyStatus::Lost => String::from("lost"),
    }
}
