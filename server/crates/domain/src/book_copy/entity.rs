use chrono::{DateTime, Utc};

pub struct BookCopy {
    pub id: i64,
    pub barcode: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_id: i64,
    pub author_name: String
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
