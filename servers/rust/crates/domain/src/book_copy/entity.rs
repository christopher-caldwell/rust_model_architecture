use chrono::{DateTime, Utc};

use crate::book::BookId;

use super::enums::BookCopyStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BookCopyId(pub i32);

pub struct BookCopy {
    pub id: BookCopyId,
    pub barcode: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_id: BookId,
    pub author_name: String,
    pub status: BookCopyStatus,
}

pub struct BookCopyCreationPayload {
    pub barcode: String,
    pub book_id: BookId,
}

pub struct BookCopyPrepared {
    pub barcode: String,
    pub book_id: BookId,
    pub status: BookCopyStatus,
}
