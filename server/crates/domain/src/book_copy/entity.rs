use chrono::{DateTime, Utc};

use crate::book::BookId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BookCopyId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookCopyStatus {
    Active,
    Maintenance,
    Lost,
}

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
    pub author_name: String,
    pub book_id: BookId,
}

pub struct BookCopyPrepared {
    pub barcode: String,
    pub author_name: String,
    pub book_id: BookId,
    pub status: BookCopyStatus,
}

impl BookCopy {
    #[must_use]
    pub fn can_be_borrowed(&self) -> bool {
        self.status == BookCopyStatus::Active
    }

    #[must_use]
    pub fn can_be_sent_to_maintenance(&self) -> bool {
        self.status == BookCopyStatus::Active
    }

    #[must_use]
    pub fn can_be_returned_from_maintenance(&self) -> bool {
        self.status == BookCopyStatus::Maintenance
    }

    #[must_use]
    pub fn can_be_marked_lost(&self) -> bool {
        self.status != BookCopyStatus::Lost
    }

    #[must_use]
    pub fn can_be_returned_from_lost(&self) -> bool {
        self.status == BookCopyStatus::Lost
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BookCopyError {
    #[error("Book cannot currently be borrowed")]
    CannotBeBorrowed,
    #[error("Book is not active and cannot be sent to maintenance")]
    CannotBeSentToMaintenance,
    #[error("Book is not currently in maintenance, and therefore cannot be returned")]
    CannotBeReturnedFromMaintenance,
    #[error("Book is already marked lost")]
    CannotMarkBookLost,
    #[error("Book is not currently lost, and cannot be returned from lost")]
    CannotBeReturnedFromLost,
}

impl BookCopyCreationPayload {
    #[must_use]
    pub fn prepare(self) -> BookCopyPrepared {
        BookCopyPrepared {
            barcode: self.barcode,
            author_name: self.author_name,
            book_id: self.book_id,
            status: BookCopyStatus::Active,
        }
    }
}
