use anyhow::Context;
use chrono::Utc;
use domain::{
    book::{Book, BookCreationPayload, BookError},
    book_copy::{BookCopy, BookCopyCreationPayload, BookCopyError},
    uow::{UnitOfWorkPort, WriteUnitOfWorkFactory},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct CatalogCommands {
    uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
}

impl CatalogCommands {
    #[must_use]
    pub fn new(uow_factory: Arc<dyn WriteUnitOfWorkFactory>) -> Self {
        Self { uow_factory }
    }

    async fn get_book_by_isbn(
        &self,
        uow: &dyn UnitOfWorkPort,
        isbn: &str,
    ) -> Result<Book, super::CommandError> {
        uow.book_write_repo()
            .get_by_isbn(isbn)
            .await
            .context("Failed to load book for write")?
            .ok_or(BookError::NotFound.into())
    }

    async fn get_book_copy_by_barcode(
        &self,
        uow: &dyn UnitOfWorkPort,
        barcode: &str,
    ) -> Result<BookCopy, super::CommandError> {
        uow.book_copy_write_repo()
            .get_by_barcode_for_update(barcode)
            .await
            .context("Failed to load book copy for write")?
            .ok_or(BookCopyError::NotFound.into())
    }

    pub async fn add_book(
        &self,
        payload: BookCreationPayload,
    ) -> Result<Book, super::CommandError> {
        let prepared = payload.prepare();
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let result = uow
            .book_write_repo()
            .create(&prepared)
            .await
            .context("Failed to add book")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }

    pub async fn add_book_copy(
        &self,
        input: super::AddBookCopyInput,
    ) -> Result<BookCopy, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let book = self.get_book_by_isbn(&*uow, &input.isbn).await?;
        let prepared = BookCopyCreationPayload {
            barcode: input.barcode,
            book_id: book.id,
        }
        .prepare();
        let result = uow
            .book_copy_write_repo()
            .create(&prepared)
            .await
            .context("Failed to add book copy")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }

    pub async fn mark_book_copy_lost(
        &self,
        barcode: String,
    ) -> Result<BookCopy, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let book_copy = self.get_book_copy_by_barcode(&*uow, &barcode).await?;
        let lost_status = book_copy.mark_lost()?;
        uow.book_copy_write_repo()
            .update_status(book_copy.id, lost_status.clone())
            .await
            .context("Failed to mark book copy lost")?;
        uow.commit().await.context("Failed to commit transaction")?;
        let updated_copy = BookCopy {
            status: lost_status,
            dt_modified: Utc::now(),
            ..book_copy
        };
        Ok(updated_copy)
    }

    pub async fn mark_book_copy_found(
        &self,
        barcode: String,
    ) -> Result<BookCopy, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let book_copy = self.get_book_copy_by_barcode(&*uow, &barcode).await?;
        let found_status = book_copy.mark_found()?;
        uow.book_copy_write_repo()
            .update_status(book_copy.id, found_status.clone())
            .await
            .context("Failed to mark book copy found")?;
        uow.commit().await.context("Failed to commit transaction")?;
        let updated_copy = BookCopy {
            status: found_status,
            dt_modified: Utc::now(),
            ..book_copy
        };
        Ok(updated_copy)
    }

    pub async fn send_book_copy_to_maintenance(
        &self,
        barcode: String,
    ) -> Result<BookCopy, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let book_copy = self.get_book_copy_by_barcode(&*uow, &barcode).await?;
        let maintenance_status = book_copy.send_to_maintenance()?;
        uow.book_copy_write_repo()
            .update_status(book_copy.id, maintenance_status.clone())
            .await
            .context("Failed to send book copy to maintenance")?;
        uow.commit().await.context("Failed to commit transaction")?;
        let updated_copy = BookCopy {
            status: maintenance_status,
            dt_modified: Utc::now(),
            ..book_copy
        };
        Ok(updated_copy)
    }

    pub async fn complete_book_copy_maintenance(
        &self,
        barcode: String,
    ) -> Result<BookCopy, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let book_copy = self.get_book_copy_by_barcode(&*uow, &barcode).await?;
        let active_status = book_copy.complete_maintenance()?;
        uow.book_copy_write_repo()
            .update_status(book_copy.id, active_status.clone())
            .await
            .context("Failed to complete book copy maintenance")?;
        uow.commit().await.context("Failed to commit transaction")?;
        let updated_copy = BookCopy {
            status: active_status,
            dt_modified: Utc::now(),
            ..book_copy
        };
        Ok(updated_copy)
    }
}
