use anyhow::Context;
use domain::{
    book_copy::{BookCopy, BookCopyError},
    loan::{Loan, LoanCreationPayload, LoanError},
    member::{Member, MemberError},
};
use std::sync::Arc;

use crate::ports::{read_repos::LoanReadRepoPort, uow::WriteUnitOfWorkFactory};

#[derive(Clone)]
pub struct LendingCommands {
    uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
    loan_read_repo: Arc<dyn LoanReadRepoPort>,
}

impl LendingCommands {
    #[must_use]
    pub fn new(
        uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
        loan_read_repo: Arc<dyn LoanReadRepoPort>,
    ) -> Self {
        Self {
            uow_factory,
            loan_read_repo,
        }
    }

    pub async fn check_out_book_copy(
        &self,
        member: Member,
        book_copy: BookCopy,
    ) -> anyhow::Result<Loan> {
        anyhow::ensure!(member.can_borrow(), MemberError::CannotBorrowWhileSuspended);
        anyhow::ensure!(book_copy.can_be_borrowed(), BookCopyError::CannotBeBorrowed);

        let payload = LoanCreationPayload {
            member_id: i64::from(member.id),
            book_copy: book_copy.id,
        };
        let prepared = payload.prepare();
        let active_loan_count = self
            .loan_read_repo
            .count_active_by_member_id(i64::from(member.id))
            .await
            .context("Failed to count active loans for member")?;
        anyhow::ensure!(
            member.can_check_out_more_books(active_loan_count),
            MemberError::LoanLimitReached
        );

        let active_loan = self
            .loan_read_repo
            .find_active_by_book_copy_id(book_copy.id)
            .await
            .context("Failed to find active loan for book copy")?;
        anyhow::ensure!(active_loan.is_none(), BookCopyError::CannotBeBorrowed);

        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .loan_write_repo()
            .create(&prepared)
            .await
            .context("Failed to check out book copy")?;
        uow.commit()
            .await
            .context("Failed to commit transaction")?;

        Ok(result)
    }

    pub async fn return_book_copy(
        &self,
        book_copy: BookCopy,
    ) -> anyhow::Result<Loan> {
        let loan = self
            .loan_read_repo
            .find_active_by_book_copy_id(book_copy.id)
            .await
            .context("Failed to find active loan for book copy")?
            .ok_or(LoanError::NoActiveLoanForBookCopy)?;
        anyhow::ensure!(loan.can_be_returned(), LoanError::CannotBeReturned);

        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .loan_write_repo()
            .end(loan.id)
            .await
            .context("Failed to return book copy")?;
        uow.commit()
            .await
            .context("Failed to commit transaction")?;

        Ok(result)
    }

    pub async fn report_lost_loaned_book_copy(
        &self,
        book_copy: BookCopy,
    ) -> anyhow::Result<BookCopy> {
        anyhow::ensure!(book_copy.can_be_marked_lost(), BookCopyError::CannotMarkBookLost);

        let loan = self
            .loan_read_repo
            .find_active_by_book_copy_id(book_copy.id)
            .await
            .context("Failed to find active loan for book copy")?
            .ok_or(LoanError::NoActiveLoanForBookCopy)?;
        anyhow::ensure!(loan.can_be_returned(), LoanError::CannotBeReturned);

        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        uow.loan_write_repo()
            .end(loan.id)
            .await
            .context("Failed to close lost loan")?;

        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, "lost")
            .await
            .context("Failed to mark lost loaned book copy")?;

        uow.commit()
            .await
            .context("Failed to commit transaction")?;

        Ok(result)
    }
}
