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
