use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{
    book_copy::BookCopyId,
    loan::{port::LoanWriteRepoPort, Loan, LoanId, LoanIdent, LoanPrepared},
    member::MemberId,
};
use sqlx::{Postgres, Transaction};
use tokio::sync::Mutex;

#[derive(sqlx::FromRow)]
pub struct LoanPreparedResult {
    pub loan_id: i32,
    pub loan_ident: String,
}

#[derive(sqlx::FromRow)]
pub struct LoanUpdatedDbRow {
    pub loan_id: i32,
    pub loan_ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_copy_id: i32,
    pub member_id: i32,
    pub dt_due: Option<DateTime<Utc>>,
    pub dt_returned: Option<DateTime<Utc>>,
}

impl TryFrom<LoanUpdatedDbRow> for Loan {
    type Error = anyhow::Error;

    fn try_from(value: LoanUpdatedDbRow) -> Result<Self> {
        Ok(Self {
            id: LoanId(value.loan_id),
            ident: LoanIdent(value.loan_ident),
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            book_copy_id: BookCopyId(value.book_copy_id),
            member_id: MemberId(
                value
                    .member_id
                    .try_into()
                    .context("member_id exceeds domain range")?,
            ),
            dt_due: value.dt_due,
            dt_returned: value.dt_returned,
        })
    }
}

pub struct LoanWriteRepoTx {
    pub tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

#[async_trait]
impl LoanWriteRepoPort for LoanWriteRepoTx {
    async fn create(&self, insert: &LoanPrepared) -> Result<Loan> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let prepared_result = sqlx::query_file_as!(
            LoanPreparedResult,
            "sql/loan/commands/create.sql",
            i32::try_from(insert.book_copy_id.0)
                .context("book_copy_id exceeds SQL integer range")?,
            i32::from(insert.member_id.0),
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to create loan")?;

        Ok(Loan {
            id: LoanId(prepared_result.loan_id),
            ident: LoanIdent(prepared_result.loan_ident),
            dt_created: Utc::now(),
            dt_modified: Utc::now(),
            book_copy_id: BookCopyId(insert.book_copy_id.0),
            member_id: MemberId(insert.member_id.0),
            dt_due: None,
            dt_returned: None,
        })
    }

    async fn end(&self, id: LoanId) -> Result<Loan> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let row = sqlx::query_file_as!(
            LoanUpdatedDbRow,
            "sql/loan/commands/end.sql",
            i32::try_from(id.0).context("loan_id exceeds SQL integer range")?,
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to end loan")?;

        row.try_into()
    }
}
