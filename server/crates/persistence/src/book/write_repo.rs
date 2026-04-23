use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use domain::book::{port::BookWriteRepoPort, Book, BookId, BookPrepared};
use sqlx::{Postgres, Transaction};
use tokio::sync::Mutex;

#[derive(sqlx::FromRow)]
pub struct BookDbRow {
    pub book_id: i32,
    pub isbn: String,
    pub dt_created: chrono::DateTime<Utc>,
    pub dt_modified: chrono::DateTime<Utc>,
    pub title: String,
    pub author_name: String,
}

impl TryFrom<BookDbRow> for Book {
    type Error = anyhow::Error;

    fn try_from(value: BookDbRow) -> Result<Self> {
        Ok(Self {
            id: BookId(
                value
                    .book_id
                    .try_into()
                    .context("book_id exceeds domain range")?,
            ),
            isbn: value.isbn,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            title: value.title,
            author_name: value.author_name,
        })
    }
}

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

    async fn get_by_isbn(&self, isbn: &str) -> Result<Option<Book>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let row = sqlx::query_file_as!(BookDbRow, "sql/book/commands/get_by_isbn.sql", isbn)
            .fetch_optional(&mut **tx)
            .await
            .context("Failed to fetch book by isbn")?;

        row.map(Book::try_from).transpose()
    }
}
