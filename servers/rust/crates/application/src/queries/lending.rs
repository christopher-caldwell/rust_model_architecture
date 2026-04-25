use anyhow::Context;
use domain::{
    loan::{port::LoanReadRepoPort, Loan},
    member::MemberIdent,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct LendingQueries {
    loan_read_repo: Arc<dyn LoanReadRepoPort>,
}

impl LendingQueries {
    #[must_use]
    pub fn new(loan_read_repo: Arc<dyn LoanReadRepoPort>) -> Self {
        Self { loan_read_repo }
    }

    pub async fn get_member_loans(&self, ident: &MemberIdent) -> anyhow::Result<Vec<Loan>> {
        self.loan_read_repo
            .get_by_member_ident(ident)
            .await
            .context("Failed to get member loans")
    }

    pub async fn get_overdue_loans(&self) -> anyhow::Result<Vec<Loan>> {
        self.loan_read_repo
            .get_overdue()
            .await
            .context("Failed to get overdue loans")
    }
}
