use domain::{book::{Book, BookCreationPayload}, book_copy::{BookCopy, BookCopyCreationPayload, BookCopyError}};
use std::sync::Arc;
use anyhow::Context;

use crate::{
    ports::{uow::WriteUnitOfWorkFactory, gen_ident::IdentGeneratorPort},
};

#[derive(Clone)]
pub struct CatalogCommands {
    uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
    ident_gen: Arc<dyn IdentGeneratorPort>
}

impl CatalogCommands {
    #[must_use]
    pub fn new(
        uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
        ident_gen: Arc<dyn IdentGeneratorPort>,
    ) -> Self {
        Self {
            uow_factory,
            ident_gen
        }
    }

    pub async fn add_book(
        &self,
        payload: BookCreationPayload,
    ) -> Result<Book, anyhow::Error> {
        let prepared = payload.prepare();
        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .book_write_repo()
            .create(&prepared)
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .context("Failed to add book")?;
        uow.commit()
            .await
            .context("Failed to commit transaction")?;

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
            .map_err(|e| anyhow::anyhow!(e))
            .context("Failed to add book copy")?;
        uow.commit()
            .await
            .context("Failed to commit transaction")?;

        Ok(result)
    }

    pub async fn mark_book_copy_lost(
        &self,
        book_copy: BookCopy,
    ) -> anyhow::Result<BookCopy> {
        let can_be_marked_lost = book_copy.can_be_marked_lost();
        if !can_be_marked_lost {
            return Err(BookCopyError::CannotMarkBookLost.into());
        }

        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, "lost")
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .context("Failed to mark book copy lost")?;
        uow.commit()
            .await
            .context("Failed to commit transaction")?;
        Ok(result)
    }
    pub async fn mark_book_copy_found(
        &self,
        book_copy: BookCopy,
    ) -> anyhow::Result<BookCopy> {
        let can_be_returned_from_lost = book_copy.can_be_returned_from_lost();
        if !can_be_returned_from_lost {
            return Err(BookCopyError::CannotBeReturnedFromLost.into());
        }
        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, "active")
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .context("Failed to mark book copy found")?;
        uow.commit()
            .await
            .context("Failed to commit transaction")?;
        Ok(result)
    }

    pub async fn send_book_copy_to_maintenance(
        &self,
        book_copy: BookCopy,
    ) -> anyhow::Result<BookCopy> {
        let can_be_sent_to_maintenance = book_copy.can_be_sent_to_maintenance();
        if !can_be_sent_to_maintenance {
            return Err(BookCopyError::CannotBeSentToMaintenance.into());
        }
        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, "maintenance")
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .context("Failed to send book copy to maintenance")?;
        uow.commit()
            .await
            .context("Failed to commit transaction")?;
        Ok(result)
    }
    pub async fn complete_book_copy_maintenance(
        &self,
        book_copy: BookCopy,
    ) -> anyhow::Result<BookCopy> {
        let can_be_returned_from_maintenance = book_copy.can_be_returned_from_maintenance();
        if !can_be_returned_from_maintenance {
            return Err(BookCopyError::CannotBeReturnedFromMaintenance.into());
        }
        let uow = self.uow_factory.build().await.context("Failed to build unit of work")?;
        let result = uow
            .book_copy_write_repo()
            .update_status(book_copy.id, "active")
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .context("Failed to complete book copy maintenance")?;
        uow.commit()
            .await
            .context("Failed to commit transaction")?;
        Ok(result)
    }
}
