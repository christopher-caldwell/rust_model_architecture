use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{
    book::BookId,
    book_copy::{port::BookCopyReadRepoPort, BookCopy, BookCopyId, BookCopyStatus},
};
use sqlx::PgPool;

use std::str::FromStr;

#[derive(sqlx::FromRow)]
pub struct BookCopyDbRow {
    pub book_copy_id: i32,
    pub barcode: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub book_id: i32,
    pub status: String,
}

impl TryFrom<BookCopyDbRow> for BookCopy {
    type Error = anyhow::Error;

    fn try_from(value: BookCopyDbRow) -> Result<Self> {
        Ok(Self {
            id: BookCopyId(value.book_copy_id),
            barcode: value.barcode,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            book_id: BookId(value.book_id),
            status: BookCopyStatus::from_str(&value.status)
                .context("Invalid book copy status in DB")?,
        })
    }
}

pub struct BookCopyReadRepoSql {
    pub pool: PgPool,
}

#[async_trait]
impl BookCopyReadRepoPort for BookCopyReadRepoSql {
    async fn get_by_id(&self, book_copy_id: BookCopyId) -> Result<Option<BookCopy>> {
        let row = sqlx::query_file_as!(
            BookCopyDbRow,
            "sql/book_copy/queries/get_by_id.sql",
            book_copy_id.0
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch book copy by id")?;

        row.map(BookCopy::try_from).transpose()
    }

    async fn get_by_barcode(&self, barcode: &str) -> Result<Option<BookCopy>> {
        let row = sqlx::query_file_as!(
            BookCopyDbRow,
            "sql/book_copy/queries/get_by_barcode.sql",
            barcode
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch book copy by barcode")?;

        row.map(BookCopy::try_from).transpose()
    }
}
