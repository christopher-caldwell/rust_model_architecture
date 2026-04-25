use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use domain::{
    book_copy::BookCopyId,
    loan::{port::LoanWriteRepoPort, Loan, LoanId, LoanIdent, LoanPrepared},
    member::MemberId,
};
use sqlx::{Postgres, Transaction};
use tokio::sync::Mutex;

use crate::loan::read_repo::{CountDbRow, LoanDbRow};

#[derive(sqlx::FromRow)]
pub struct LoanPreparedResult {
    pub loan_id: i32,
    pub loan_ident: String,
}

pub struct LoanWriteRepoTx {
    pub tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

#[async_trait]
impl LoanWriteRepoPort for LoanWriteRepoTx {
    async fn create(&self, insert: &LoanPrepared) -> Result<Loan> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let book_copy_id = i32::try_from(insert.book_copy_id.0)
            .context("book_copy_id exceeds SQL integer range")?;
        let member_id = i32::from(insert.member_id.0);
        let prepared_result = sqlx::query_file_as!(
            LoanPreparedResult,
            "sql/loan/commands/create.sql",
            book_copy_id,
            member_id,
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to create loan")?;

        let now = Utc::now();
        let created_loan = Loan {
            id: LoanId(prepared_result.loan_id),
            ident: LoanIdent(prepared_result.loan_ident),
            dt_created: now,
            dt_modified: now,
            book_copy_id: insert.book_copy_id,
            member_id: insert.member_id,
            dt_due: None,
            dt_returned: None,
        };
        Ok(created_loan)
    }

    async fn end(&self, id: LoanId) -> Result<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let loan_id = i32::try_from(id.0).context("loan_id exceeds SQL integer range")?;
        sqlx::query_file!("sql/loan/commands/end.sql", loan_id,)
            .execute(&mut **tx)
            .await
            .context("Failed to end loan")?;

        Ok(())
    }

    async fn find_active_by_book_copy_id_for_update(&self, id: BookCopyId) -> Result<Option<Loan>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let book_copy_id = i32::try_from(id.0).context("book_copy_id exceeds SQL integer range")?;
        let row = sqlx::query_file_as!(
            LoanDbRow,
            "sql/loan/commands/find_active_by_book_copy_id_for_update.sql",
            book_copy_id
        )
        .fetch_optional(&mut **tx)
        .await
        .context("Failed to find active loan by book copy id")?;

        row.map(Loan::try_from).transpose()
    }

    async fn count_active_by_member_id(&self, id: MemberId) -> Result<i64> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let row = sqlx::query_file_as!(
            CountDbRow,
            "sql/loan/commands/count_active_by_member_id.sql",
            i32::from(id.0)
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to count active loans by member id")?;

        Ok(row.count)
    }
}
