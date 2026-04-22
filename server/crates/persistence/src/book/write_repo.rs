use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use domain::book::{port::BookWriteRepoPort, Book, BookId, BookPrepared};
use sqlx::{Postgres, Transaction};
use tokio::sync::Mutex;

#[derive(sqlx::FromRow)]
pub struct BookPreparedResult {
    pub book_id: i32,
}

pub struct BookWriteRepoTx {
    pub tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

#[async_trait]
impl BookWriteRepoPort for BookWriteRepoTx {
    async fn create(&self, insert: &BookPrepared) -> Result<Book> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let prepared_result = sqlx::query_file_as!(
            BookPreparedResult,
            "sql/book/commands/create.sql",
            insert.isbn,
            insert.title,
            insert.author_name,
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to create book")?;

        Ok(Book {
            id: BookId(
                prepared_result
                    .book_id
                    .try_into()
                    .context("book_id exceeds domain range")?,
            ),
            isbn: insert.isbn.clone(),
            dt_created: Utc::now(),
            dt_modified: Utc::now(),
            title: insert.title.clone(),
            author_name: insert.author_name.clone(),
        })
    }
}
