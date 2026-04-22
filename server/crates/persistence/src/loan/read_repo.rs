use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{
    book_copy::BookCopyId,
    loan::{port::LoanReadRepoPort, Loan, LoanId, LoanIdent},
    member::{MemberId, MemberIdent},
};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use tokio::sync::Mutex;

#[derive(sqlx::FromRow)]
pub struct LoanDbRow {
    pub loan_id: i32,
    pub loan_ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_copy_id: i32,
    pub member_id: i32,
    pub dt_due: Option<DateTime<Utc>>,
    pub dt_returned: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
pub struct CountDbRow {
    pub count: i64,
}

impl TryFrom<LoanDbRow> for Loan {
    type Error = anyhow::Error;

    fn try_from(value: LoanDbRow) -> Result<Self> {
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

async fn get_by_member_ident_with<'e, E>(executor: E, ident: &MemberIdent) -> Result<Vec<Loan>>
where
    E: Executor<'e, Database = Postgres>,
{
    let ident_str: String = ident.clone().into();
    let rows = sqlx::query_file_as!(
        LoanDbRow,
        "sql/loan/queries/get_by_member_ident.sql",
        ident_str
    )
    .fetch_all(executor)
    .await
    .context("Failed to fetch loans by member ident")?;

    rows.into_iter().map(Loan::try_from).collect()
}

async fn get_overdue_with<'e, E>(executor: E) -> Result<Vec<Loan>>
where
    E: Executor<'e, Database = Postgres>,
{
    let rows = sqlx::query_file_as!(LoanDbRow, "sql/loan/queries/get_overdue.sql")
        .fetch_all(executor)
        .await
        .context("Failed to fetch overdue loans")?;

    rows.into_iter().map(Loan::try_from).collect()
}

async fn find_active_by_book_copy_id_with<'e, E>(
    executor: E,
    id: BookCopyId,
) -> Result<Option<Loan>>
where
    E: Executor<'e, Database = Postgres>,
{
    let row = sqlx::query_file_as!(
        LoanDbRow,
        "sql/loan/queries/find_active_by_book_copy_id.sql",
        i32::try_from(id.0).context("book_copy_id exceeds SQL integer range")?
    )
    .fetch_optional(executor)
    .await
    .context("Failed to find active loan by book copy id")?;

    row.map(Loan::try_from).transpose()
}

async fn count_active_by_member_id_with<'e, E>(executor: E, id: MemberId) -> Result<i64>
where
    E: Executor<'e, Database = Postgres>,
{
    let row = sqlx::query_file_as!(
        CountDbRow,
        "sql/loan/queries/count_active_by_member_id.sql",
        i32::from(id.0)
    )
    .fetch_one(executor)
    .await
    .context("Failed to count active loans by member id")?;

    Ok(row.count)
}

pub struct LoanReadRepoSql {
    pub pool: PgPool,
}

pub struct LoanReadRepoTx {
    pub tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

#[async_trait]
impl LoanReadRepoPort for LoanReadRepoSql {
    async fn get_by_member_ident(&self, ident: &MemberIdent) -> Result<Vec<Loan>> {
        get_by_member_ident_with(&self.pool, ident).await
    }

    async fn get_overdue(&self) -> Result<Vec<Loan>> {
        get_overdue_with(&self.pool).await
    }

    async fn find_active_by_book_copy_id(&self, id: BookCopyId) -> Result<Option<Loan>> {
        find_active_by_book_copy_id_with(&self.pool, id).await
    }

    async fn count_active_by_member_id(&self, id: MemberId) -> Result<i64> {
        count_active_by_member_id_with(&self.pool, id).await
    }
}

#[async_trait]
impl LoanReadRepoPort for LoanReadRepoTx {
    async fn get_by_member_ident(&self, ident: &MemberIdent) -> Result<Vec<Loan>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        get_by_member_ident_with(&mut **tx, ident).await
    }

    async fn get_overdue(&self) -> Result<Vec<Loan>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        get_overdue_with(&mut **tx).await
    }

    async fn find_active_by_book_copy_id(&self, id: BookCopyId) -> Result<Option<Loan>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        find_active_by_book_copy_id_with(&mut **tx, id).await
    }

    async fn count_active_by_member_id(&self, id: MemberId) -> Result<i64> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        count_active_by_member_id_with(&mut **tx, id).await
    }
}
