use domain::{book::BookError, book_copy::BookCopyError, loan::LoanError, member::MemberError};

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("{entity} not found")]
    NotFound { entity: &'static str },

    #[error("{message}")]
    Conflict { message: String },

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl CommandError {
    #[must_use]
    pub fn not_found(entity: &'static str) -> Self {
        Self::NotFound { entity }
    }

    #[must_use]
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict {
            message: message.into(),
        }
    }
}

impl From<BookError> for CommandError {
    fn from(value: BookError) -> Self {
        match value {
            BookError::NotFound => CommandError::not_found("Book"),
        }
    }
}

impl From<BookCopyError> for CommandError {
    fn from(value: BookCopyError) -> Self {
        match value {
            BookCopyError::NotFound => CommandError::not_found("Book copy"),
            other => CommandError::conflict(other.to_string()),
        }
    }
}

impl From<MemberError> for CommandError {
    fn from(value: MemberError) -> Self {
        match value {
            MemberError::NotFound => CommandError::not_found("Member"),
            other => CommandError::conflict(other.to_string()),
        }
    }
}

impl From<LoanError> for CommandError {
    fn from(value: LoanError) -> Self {
        match value {
            LoanError::NoActiveLoanForBookCopy => CommandError::conflict(value.to_string()),
            LoanError::CannotBeReturned => CommandError::conflict(value.to_string()),
        }
    }
}
