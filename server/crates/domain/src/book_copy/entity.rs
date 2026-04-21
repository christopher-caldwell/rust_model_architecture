use chrono::{DateTime, Utc};

pub struct BookCopy {
    pub id: i64,
    pub barcode: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_id: i64,
    pub author_name: String,
    pub status: String
}

pub struct BookCopyCreationPayload {
    pub barcode: String,
    pub author_name: String,
    pub book_id: i64
}

pub struct BookCopyPrepared {
    pub barcode: String,
    pub author_name: String,
    pub book_id: i64,
    pub status: &'static str
}

impl BookCopy {
    #[must_use]
    pub fn can_be_borrowed(&self) -> bool {
        self.status == "active"
    }
    #[must_use]
    pub fn can_be_sent_to_maintenance(&self) -> bool {
        return self.status != "loaned" && self.status != "lost"
    }
    #[must_use]
    pub fn can_be_returned_from_maintenance(&self) -> bool {
        return self.status == "maintenance"
    }
    #[must_use]
    pub fn can_be_marked_lost(&self) -> bool {
        return self.status != "lost"
    }
    #[must_use]
    pub fn can_be_returned_from_lost(&self) -> bool {
        return self.status == "lost"
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BookCopyError {
    #[error("Book cannot currently be borrowed")]
    CannotBeBorrowed,
    #[error("Book is loaned or lost and cannot be sent to maintenance")]
    CannotBeSentToMaintenance,
    #[error("Book is not currently in maintenance, and therefore cannot be returned")]
    CannotBeReturnedFromMaintenance,
    #[error("Book is already marked lost.")]
    CannotMarkBookLost,
    #[error("Book is not currently lost, and cannot be returned from lost")]
    CannotBeReturnedFromLost,
}

impl BookCopyCreationPayload {
    #[must_use]
    pub fn prepare(self) -> BookCopyPrepared {
        // Additional logic can be done here if needed
        let status: &str = "active";
        BookCopyPrepared {
            barcode: self.barcode,
            author_name: self.author_name,
            book_id: self.book_id,
            status
        }
    }
}
