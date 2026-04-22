use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BookId(pub i32);

pub struct Book {
    pub id: BookId,
    pub isbn: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub title: String,
    pub author_name: String,
}

pub struct BookCreationPayload {
    pub isbn: String,
    pub title: String,
    pub author_name: String,
}

pub struct BookPrepared {
    pub isbn: String,
    pub title: String,
    pub author_name: String,
}

impl BookCreationPayload {
    #[must_use]
    pub fn prepare(self) -> BookPrepared {
        BookPrepared {
            isbn: self.isbn,
            title: self.title,
            author_name: self.author_name,
        }
    }
}
