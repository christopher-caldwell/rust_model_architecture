use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{
    book::BookId,
    book_copy::{
        port::BookCopyWriteRepoPort, BookCopy, BookCopyId, BookCopyPrepared, BookCopyStatus,
    },
};
use sqlx::{Postgres, Transaction};
use tokio::sync::Mutex;

use crate::book_copy::{book_copy_status_ident, parse_book_copy_status};

#[derive(sqlx::FromRow)]
pub struct BookCopyPreparedResult {
    pub book_copy_id: i32,
}

#[derive(sqlx::FromRow)]
pub struct BookCopyUpdatedDbRow {
    pub book_copy_id: i32,
    pub barcode: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_id: i32,
    pub author_name: String,
    pub status: String,
}

impl TryFrom<BookCopyUpdatedDbRow> for BookCopy {
    type Error = anyhow::Error;

    fn try_from(value: BookCopyUpdatedDbRow) -> Result<Self> {
        Ok(Self {
            id: BookCopyId(value.book_copy_id),
            barcode: value.barcode,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            book_id: BookId(
                value
                    .book_id
                    .try_into()
                    .context("book_id exceeds domain range")?,
            ),
            author_name: value.author_name,
            status: parse_book_copy_status(&value.status)?,
        })
    }
}

pub struct BookCopyWriteRepoTx {
    pub tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

#[async_trait]
impl BookCopyWriteRepoPort for BookCopyWriteRepoTx {
    async fn create(&self, insert: &BookCopyPrepared) -> Result<BookCopy> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let prepared_result = sqlx::query_file_as!(
            BookCopyPreparedResult,
            "sql/book_copy/commands/create.sql",
            i32::from(insert.book_id.0),
            book_copy_status_ident(&insert.status),
            insert.barcode,
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to create book copy")?;

        Ok(BookCopy {
            id: BookCopyId(prepared_result.book_copy_id),
            barcode: insert.barcode.clone(),
            dt_created: Utc::now(),
            dt_modified: Utc::now(),
            book_id: BookId(insert.book_id.0),
            author_name: insert.author_name.clone(),
            status: insert.status.clone(),
        })
    }

    async fn update_status(&self, id: BookCopyId, status: BookCopyStatus) -> Result<BookCopy> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let row = sqlx::query_file_as!(
            BookCopyUpdatedDbRow,
            "sql/book_copy/commands/update_status.sql",
            i32::try_from(id.0).context("book_copy_id exceeds SQL integer range")?,
            book_copy_status_ident(&status),
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to update book copy status")?;

        row.try_into()
    }
}
