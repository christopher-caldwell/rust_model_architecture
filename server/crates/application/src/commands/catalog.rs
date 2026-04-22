use anyhow::Context;
use domain::{
    book::{Book, BookCreationPayload},
    book_copy::{BookCopy, BookCopyCreationPayload, BookCopyError, BookCopyStatus},
    uow::WriteUnitOfWorkFactory,
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

    async fn change_book_copy_status(
        &self,
        book_copy: BookCopy,
        new_status: BookCopyStatus,
        added_context: &'static str,
    ) -> anyhow::Result<BookCopy> {
        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, new_status)
            .await
            .context(added_context)?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }

    pub async fn add_book(&self, payload: BookCreationPayload) -> Result<Book, anyhow::Error> {
        let prepared = payload.prepare();
        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
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
        payload: BookCopyCreationPayload,
    ) -> Result<BookCopy, anyhow::Error> {
        let prepared = payload.prepare();
        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .book_copy_write_repo()
            .create(&prepared)
            .await
            .context("Failed to add book copy")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }

    pub async fn mark_book_copy_lost(&self, book_copy: BookCopy) -> anyhow::Result<BookCopy> {
        anyhow::ensure!(book_copy.can_be_marked_lost(), BookCopyError::CannotMarkBookLost);
        self.change_book_copy_status(book_copy, BookCopyStatus::Lost, "Failed to mark book copy lost").await
    }

    pub async fn mark_book_copy_found(&self, book_copy: BookCopy) -> anyhow::Result<BookCopy> {
        anyhow::ensure!(book_copy.can_be_returned_from_lost(), BookCopyError::CannotBeReturnedFromLost);
        self.change_book_copy_status(book_copy, BookCopyStatus::Active, "Failed to mark book copy found").await
    }

    pub async fn send_book_copy_to_maintenance(
        &self,
        book_copy: BookCopy,
    ) -> anyhow::Result<BookCopy> {
        anyhow::ensure!(book_copy.can_be_sent_to_maintenance(), BookCopyError::CannotBeSentToMaintenance);
        self.change_book_copy_status(book_copy, BookCopyStatus::Maintenance, "Failed to send book copy to maintenance").await
    }

    pub async fn complete_book_copy_maintenance(
        &self,
        book_copy: BookCopy,
    ) -> anyhow::Result<BookCopy> {
        anyhow::ensure!(book_copy.can_be_returned_from_maintenance(), BookCopyError::CannotBeReturnedFromMaintenance);
        self.change_book_copy_status(book_copy, BookCopyStatus::Active, "Failed to complete book copy maintenance").await
    }
}
