use super::entity::{BookCreationPayload, BookPrepared};

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
