use anyhow::Context;
use domain::loan::Loan;
use std::sync::Arc;

use crate::ports::read_repos::LoanReadRepoPort;

#[derive(Clone)]
pub struct LendingQueries {
    loan_read_repo: Arc<dyn LoanReadRepoPort>,
}

impl LendingQueries {
    #[must_use]
    pub fn new(
        loan_read_repo: Arc<dyn LoanReadRepoPort>,
    ) -> Self {
        Self { loan_read_repo }
    }

    pub async fn get_member_loans(
        &self,
        member_id: i64,
    ) -> anyhow::Result<Vec<Loan>> {
        let result = self
            .loan_read_repo
            .get_member_loans(member_id)
            .await
            .context("Failed to get member loans")?;

        Ok(result)
    }

    pub async fn get_overdue_loans(
        &self,
    ) -> anyhow::Result<Vec<Loan>> {
        let result = self
            .loan_read_repo
            .get_overdue_loans()
            .await
            .context("Failed to get overdue loans")?;

        Ok(result)
    }
}
