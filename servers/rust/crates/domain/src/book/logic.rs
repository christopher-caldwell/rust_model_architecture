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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_passes_through_fields() {
        let prepared = BookCreationPayload {
            isbn: "9780134685991".to_string(),
            title: "Effective Java".to_string(),
            author_name: "Joshua Bloch".to_string(),
        }
        .prepare();

        assert_eq!(prepared.isbn, "9780134685991");
        assert_eq!(prepared.title, "Effective Java");
        assert_eq!(prepared.author_name, "Joshua Bloch");
    }
}
