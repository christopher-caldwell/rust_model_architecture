use anyhow::Context;
use domain::{
    book::{Book, BookCreationPayload, BookError},
    book_copy::{BookCopy, BookCopyCreationPayload, BookCopyError, BookCopyStatus},
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
            author_name: input.author_name,
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
        if !book_copy.can_be_marked_lost() {
            return Err(BookCopyError::CannotMarkBookLost.into());
        }
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, BookCopyStatus::Lost)
            .await
            .context("Failed to mark book copy lost")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
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
        if !book_copy.can_be_returned_from_lost() {
            return Err(BookCopyError::CannotBeReturnedFromLost.into());
        }
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, BookCopyStatus::Active)
            .await
            .context("Failed to mark book copy found")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
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
        if !book_copy.can_be_sent_to_maintenance() {
            return Err(BookCopyError::CannotBeSentToMaintenance.into());
        }
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, BookCopyStatus::Maintenance)
            .await
            .context("Failed to send book copy to maintenance")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
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
        if !book_copy.can_be_returned_from_maintenance() {
            return Err(BookCopyError::CannotBeReturnedFromMaintenance.into());
        }
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, BookCopyStatus::Active)
            .await
            .context("Failed to complete book copy maintenance")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }
}
