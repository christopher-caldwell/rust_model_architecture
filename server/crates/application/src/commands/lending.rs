use anyhow::Context;
use domain::{
    book_copy::{BookCopy, BookCopyError, BookCopyStatus},
    loan::{Loan, LoanCreationPayload, LoanError},
    member::{Member, MemberError},
    uow::{UnitOfWorkPort, WriteUnitOfWorkFactory},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct LendingCommands {
    uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
}

impl LendingCommands {
    #[must_use]
    pub fn new(uow_factory: Arc<dyn WriteUnitOfWorkFactory>) -> Self {
        Self { uow_factory }
    }

    async fn get_member_by_ident(
        &self,
        uow: &dyn UnitOfWorkPort,
        member_ident: &str,
    ) -> Result<Member, super::CommandError> {
        let ident = domain::member::MemberIdent(member_ident.to_owned());
        uow.membership_write_repo()
            .get_by_ident_for_update(&ident)
            .await
            .context("Failed to load member for write")?
            .ok_or(MemberError::NotFound.into())
    }

    async fn get_book_copy_by_barcode(
        &self,
        uow: &dyn UnitOfWorkPort,
        barcode: &str,
    ) -> Result<BookCopy, super::CommandError> {
        uow.book_copy_write_repo()
            .get_by_barcode_for_update(barcode)
            .await
            .context("Failed to load book copy for write")?
            .ok_or(BookCopyError::NotFound.into())
    }

    async fn load_active_loan_for_copy(
        &self,
        uow: &dyn UnitOfWorkPort,
        book_copy_id: domain::book_copy::BookCopyId,
    ) -> Result<Option<Loan>, super::CommandError> {
        Ok(uow
            .loan_write_repo()
            .find_active_by_book_copy_id_for_update(book_copy_id)
            .await
            .context("Failed to find active loan for book copy")?)
    }

    pub async fn check_out_book_copy(
        &self,
        input: super::CheckOutBookCopyInput,
    ) -> Result<Loan, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let member = self.get_member_by_ident(&*uow, &input.member_ident).await?;
        let book_copy = self
            .get_book_copy_by_barcode(&*uow, &input.book_copy_barcode)
            .await?;

        if !member.can_borrow() {
            return Err(MemberError::CannotBorrowWhileSuspended.into());
        }
        if !book_copy.can_be_borrowed() {
            return Err(BookCopyError::CannotBeBorrowed.into());
        }

        let active_loan_count = uow
            .loan_write_repo()
            .count_active_by_member_id(member.id)
            .await
            .context("Failed to count active loans for member")?;
        if !member.can_check_out_more_books(active_loan_count as i16) {
            return Err(MemberError::LoanLimitReached.into());
        }

        let active_loan = self.load_active_loan_for_copy(&*uow, book_copy.id).await?;
        if active_loan.is_some() {
            return Err(BookCopyError::CannotBeBorrowed.into());
        }

        let prepared = LoanCreationPayload {
            member_id: member.id,
            book_copy_id: book_copy.id,
        }
        .prepare();
        let result = uow
            .loan_write_repo()
            .create(&prepared)
            .await
            .context("Failed to check out book copy")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }

    pub async fn return_book_copy(&self, barcode: String) -> Result<Loan, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let book_copy = self.get_book_copy_by_barcode(&*uow, &barcode).await?;
        let loan = self
            .load_active_loan_for_copy(&*uow, book_copy.id)
            .await?
            .ok_or(LoanError::NoActiveLoanForBookCopy)?;
        if !loan.can_be_returned() {
            return Err(LoanError::CannotBeReturned.into());
        }
        let result = uow
            .loan_write_repo()
            .end(loan.id)
            .await
            .context("Failed to return book copy")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }

    pub async fn report_lost_loaned_book_copy(
        &self,
        barcode: String,
    ) -> Result<BookCopy, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let book_copy = self.get_book_copy_by_barcode(&*uow, &barcode).await?;
        if !book_copy.can_be_marked_lost() {
            return Err(BookCopyError::CannotMarkBookLost.into());
        }
        let loan = self
            .load_active_loan_for_copy(&*uow, book_copy.id)
            .await?
            .ok_or(LoanError::NoActiveLoanForBookCopy)?;
        if !loan.can_be_returned() {
            return Err(LoanError::CannotBeReturned.into());
        }
        uow.loan_write_repo()
            .end(loan.id)
            .await
            .context("Failed to close lost loan")?;
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, BookCopyStatus::Lost)
            .await
            .context("Failed to mark book copy as lost")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }
}
