use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use domain::book::{port::BookWriteRepoPort, Book, BookId, BookPrepared};
use sqlx::{Postgres, Transaction};
use tokio::sync::Mutex;

use crate::book::read_repo::BookDbRow;

#[derive(sqlx::FromRow)]
pub struct BookCreateResult {
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
        let created_book = sqlx::query_file_as!(
            BookCreateResult,
            "sql/book/commands/create.sql",
            insert.isbn,
            insert.title,
            insert.author_name,
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to create book")?;

        let now = Utc::now();
        let created_book = Book {
            id: BookId(created_book.book_id),
            isbn: insert.isbn.clone(),
            dt_created: now,
            dt_modified: now,
            title: insert.title.clone(),
            author_name: insert.author_name.clone(),
        };
        Ok(created_book)
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
