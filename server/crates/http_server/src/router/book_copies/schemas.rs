use chrono::{DateTime, Utc};
use domain::book_copy::BookCopy;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const BOOK_COPIES_TAG: &str = "catalog";
pub const BOOK_COPIES_PATH: &str = "/book-copies";
pub const BOOK_COPY_BY_ID_PATH: &str = "/book-copies/{barcode}";
pub const BOOK_COPY_LOST_PATH: &str = "/book-copies/{barcode}/lost";
pub const BOOK_COPY_MAINTENANCE_PATH: &str = "/book-copies/{barcode}/maintenance";
pub const BOOK_COPY_RETURN_PATH: &str = "/book-copies/{barcode}/return";
pub const BOOK_COPY_REPORT_LOSS_PATH: &str = "/book-copies/{barcode}/report-loss";

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
            status: value.status.to_string(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateBookCopyRequestBody {
    pub barcode: String,
}
