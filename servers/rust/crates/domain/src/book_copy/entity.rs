use chrono::{DateTime, Utc};

use crate::book::BookId;

use super::enums::BookCopyStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BookCopyId(pub i32);

#[derive(Debug, Clone)]
pub struct BookCopy {
    pub id: BookCopyId,
    pub barcode: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_id: BookId,
    pub status: BookCopyStatus,
}

#[derive(Debug, Clone)]
pub struct BookCopyCreationPayload {
    pub barcode: String,
    pub book_id: BookId,
}

#[derive(Debug, Clone)]
pub struct BookCopyPrepared {
    pub barcode: String,
    pub book_id: BookId,
    pub status: BookCopyStatus,
}
