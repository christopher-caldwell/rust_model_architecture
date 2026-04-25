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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    use crate::book::BookId;
    use super::super::entity::BookCopyId;

    fn copy_with_status(status: BookCopyStatus) -> BookCopy {
        BookCopy {
            id: BookCopyId(1),
            barcode: "BC-001".to_string(),
            dt_created: Utc::now(),
            dt_modified: Utc::now(),
            book_id: BookId(1),
            status,
        }
    }

    // --- ensure_can_be_borrowed ---

    #[test]
    fn active_copy_can_be_borrowed() {
        let copy = copy_with_status(BookCopyStatus::Active);
        assert!(copy.ensure_can_be_borrowed().is_ok());
    }

    #[test]
    fn maintenance_copy_cannot_be_borrowed() {
        let copy = copy_with_status(BookCopyStatus::Maintenance);
        assert!(matches!(
            copy.ensure_can_be_borrowed(),
            Err(BookCopyError::CannotBeBorrowed)
        ));
    }

    #[test]
    fn lost_copy_cannot_be_borrowed() {
        let copy = copy_with_status(BookCopyStatus::Lost);
        assert!(matches!(
            copy.ensure_can_be_borrowed(),
            Err(BookCopyError::CannotBeBorrowed)
        ));
    }

    // --- send_to_maintenance ---

    #[test]
    fn active_copy_can_be_sent_to_maintenance() {
        let copy = copy_with_status(BookCopyStatus::Active);
        assert_eq!(copy.send_to_maintenance().unwrap(), BookCopyStatus::Maintenance);
    }

    #[test]
    fn maintenance_copy_cannot_be_sent_to_maintenance_again() {
        let copy = copy_with_status(BookCopyStatus::Maintenance);
        assert!(matches!(
            copy.send_to_maintenance(),
            Err(BookCopyError::CannotBeSentToMaintenance)
        ));
    }

    #[test]
    fn lost_copy_cannot_be_sent_to_maintenance() {
        let copy = copy_with_status(BookCopyStatus::Lost);
        assert!(matches!(
            copy.send_to_maintenance(),
            Err(BookCopyError::CannotBeSentToMaintenance)
        ));
    }

    // --- complete_maintenance ---

    #[test]
    fn maintenance_copy_can_complete_maintenance() {
        let copy = copy_with_status(BookCopyStatus::Maintenance);
        assert_eq!(copy.complete_maintenance().unwrap(), BookCopyStatus::Active);
    }

    #[test]
    fn active_copy_cannot_complete_maintenance() {
        let copy = copy_with_status(BookCopyStatus::Active);
        assert!(matches!(
            copy.complete_maintenance(),
            Err(BookCopyError::CannotBeReturnedFromMaintenance)
        ));
    }

    // --- mark_lost ---

    #[test]
    fn active_copy_can_be_marked_lost() {
        let copy = copy_with_status(BookCopyStatus::Active);
        assert_eq!(copy.mark_lost().unwrap(), BookCopyStatus::Lost);
    }

    #[test]
    fn maintenance_copy_can_be_marked_lost() {
        let copy = copy_with_status(BookCopyStatus::Maintenance);
        assert_eq!(copy.mark_lost().unwrap(), BookCopyStatus::Lost);
    }

    #[test]
    fn lost_copy_cannot_be_marked_lost_again() {
        let copy = copy_with_status(BookCopyStatus::Lost);
        assert!(matches!(
            copy.mark_lost(),
            Err(BookCopyError::CannotMarkBookLost)
        ));
    }

    // --- mark_found ---

    #[test]
    fn lost_copy_can_be_marked_found() {
        let copy = copy_with_status(BookCopyStatus::Lost);
        assert_eq!(copy.mark_found().unwrap(), BookCopyStatus::Active);
    }

    #[test]
    fn active_copy_cannot_be_marked_found() {
        let copy = copy_with_status(BookCopyStatus::Active);
        assert!(matches!(
            copy.mark_found(),
            Err(BookCopyError::CannotBeReturnedFromLost)
        ));
    }

    // --- prepare ---

    #[test]
    fn prepare_sets_active_status() {
        let payload = BookCopyCreationPayload {
            barcode: "BC-002".to_string(),
            book_id: BookId(2),
        };
        let prepared = payload.prepare();
        assert_eq!(prepared.status, BookCopyStatus::Active);
        assert_eq!(prepared.barcode, "BC-002");
    }
}
