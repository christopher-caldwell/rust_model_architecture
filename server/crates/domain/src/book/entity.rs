use chrono::{DateTime, Utc};

pub struct Book {
    pub id: i16,
    pub isbn: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub title: String,
    pub author_name: i16
}

pub struct BookCreationPayload {
    pub isbn: String,
    pub title: String,
    pub author_name: String
}

pub struct BookPrepared {
    pub isbn: String,
    pub title: String,
    pub author_name: String
}

impl BookCreationPayload {
    #[must_use]
    pub fn prepare(self) -> BookPrepared {
        // Additional logic can be done here if needed
        BookPrepared {
            isbn: self.isbn,
            title: self.title,
            author_name: self.author_name,
        }
    }
}
