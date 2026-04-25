use super::entity::{BookCopy, BookCopyCreationPayload, BookCopyPrepared};
use super::enums::BookCopyStatus;
use super::errors::BookCopyError;

impl BookCopy {
    /// Returns `true` when this copy is physically borrowable (not lost, not in maintenance).
    /// Full borrowability also requires checking that no active loan exists for this copy.
    #[must_use]
    fn is_borrowable(&self) -> bool {
        self.status == BookCopyStatus::Active
    }

    /// Guard: ensures copy is in a borrowable state for borrowing.
    pub fn ensure_can_be_borrowed(&self) -> Result<(), BookCopyError> {
        if !self.is_borrowable() {
            return Err(BookCopyError::CannotBeBorrowed);
        }
        Ok(())
    }

    /// Transition: Active -> Maintenance.
    pub fn send_to_maintenance(&self) -> Result<BookCopyStatus, BookCopyError> {
        if self.status != BookCopyStatus::Active {
            return Err(BookCopyError::CannotBeSentToMaintenance);
        }
        Ok(BookCopyStatus::Maintenance)
    }

    /// Transition: Maintenance -> Active.
    pub fn complete_maintenance(&self) -> Result<BookCopyStatus, BookCopyError> {
        if self.status != BookCopyStatus::Maintenance {
            return Err(BookCopyError::CannotBeReturnedFromMaintenance);
        }
        Ok(BookCopyStatus::Active)
    }

    /// Transition: any non-Lost -> Lost.
    pub fn mark_lost(&self) -> Result<BookCopyStatus, BookCopyError> {
        if self.status == BookCopyStatus::Lost {
            return Err(BookCopyError::CannotMarkBookLost);
        }
        Ok(BookCopyStatus::Lost)
    }

    /// Transition: Lost -> Active.
    pub fn mark_found(&self) -> Result<BookCopyStatus, BookCopyError> {
        if self.status != BookCopyStatus::Lost {
            return Err(BookCopyError::CannotBeReturnedFromLost);
        }
        Ok(BookCopyStatus::Active)
    }
}

impl BookCopyCreationPayload {
    #[must_use]
    pub fn prepare(self) -> BookCopyPrepared {
        BookCopyPrepared {
            barcode: self.barcode,
            book_id: self.book_id,
            status: BookCopyStatus::Active,
        }
    }
}
