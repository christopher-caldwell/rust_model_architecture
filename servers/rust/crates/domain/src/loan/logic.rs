use super::entity::{Loan, LoanCreationPayload, LoanPrepared};
use super::errors::LoanError;

impl Loan {
    #[must_use]
    fn can_be_returned(&self) -> bool {
        self.dt_returned.is_none()
    }

    /// Guard: ensures loan has not already been returned.
    pub fn ensure_can_be_returned(&self) -> Result<(), LoanError> {
        if !self.can_be_returned() {
            return Err(LoanError::CannotBeReturned);
        }
        Ok(())
    }
}

impl LoanCreationPayload {
    #[must_use]
    pub fn prepare(self) -> LoanPrepared {
        LoanPrepared {
            member_id: self.member_id,
            book_copy_id: self.book_copy_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    use crate::{book_copy::BookCopyId, member::MemberId};
    use super::super::entity::{LoanId, LoanIdent};

    fn active_loan() -> Loan {
        Loan {
            id: LoanId(1),
            ident: LoanIdent("LOAN-0001".to_string()),
            dt_created: Utc::now(),
            dt_modified: Utc::now(),
            book_copy_id: BookCopyId(1),
            member_id: MemberId(1),
            dt_due: None,
            dt_returned: None,
        }
    }

    fn returned_loan() -> Loan {
        Loan {
            dt_returned: Some(Utc::now()),
            ..active_loan()
        }
    }

    #[test]
    fn active_loan_can_be_returned() {
        let loan = active_loan();
        assert!(loan.ensure_can_be_returned().is_ok());
    }

    #[test]
    fn returned_loan_cannot_be_returned_again() {
        let loan = returned_loan();
        assert!(matches!(
            loan.ensure_can_be_returned(),
            Err(LoanError::CannotBeReturned)
        ));
    }

    #[test]
    fn prepare_passes_through_ids() {
        let payload = LoanCreationPayload {
            member_id: MemberId(42),
            book_copy_id: BookCopyId(7),
        };
        let prepared = payload.prepare();
        assert_eq!(prepared.member_id, MemberId(42));
        assert_eq!(prepared.book_copy_id, BookCopyId(7));
    }
}
