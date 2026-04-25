use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use domain::uow::{UnitOfWorkPort, WriteUnitOfWorkFactory};
use sqlx::{PgPool, Postgres, Transaction};
use tokio::sync::Mutex;

use crate::{
    book::write_repo::BookWriteRepoTx, book_copy::write_repo::BookCopyWriteRepoTx,
    loan::write_repo::LoanWriteRepoTx, member::write_repo::MemberWriteRepoTx,
};

pub struct SqlUnitOfWork {
    tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
    book_write_repo: BookWriteRepoTx,
    book_copy_write_repo: BookCopyWriteRepoTx,
    membership_write_repo: MemberWriteRepoTx,
    loan_write_repo: LoanWriteRepoTx,
}

#[async_trait]
impl UnitOfWorkPort for SqlUnitOfWork {
    fn book_write_repo(&self) -> &dyn domain::book::port::BookWriteRepoPort {
        &self.book_write_repo
    }

    fn book_copy_write_repo(&self) -> &dyn domain::book_copy::port::BookCopyWriteRepoPort {
        &self.book_copy_write_repo
    }

    fn membership_write_repo(&self) -> &dyn domain::member::port::MemberWriteRepoPort {
        &self.membership_write_repo
    }

    fn loan_write_repo(&self) -> &dyn domain::loan::port::LoanWriteRepoPort {
        &self.loan_write_repo
    }

    async fn commit(self: Box<Self>) -> anyhow::Result<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.take().context("Transaction already consumed")?;
        tx.commit().await.context("Failed to commit transaction")
    }
}

pub struct SqlWriteUnitOfWorkFactory {
    pub pool: PgPool,
}

#[async_trait]
impl WriteUnitOfWorkFactory for SqlWriteUnitOfWorkFactory {
    async fn build(&self) -> anyhow::Result<Box<dyn UnitOfWorkPort>> {
        let tx = self
            .pool
            .begin()
            .await
            .context("Failed to begin transaction")?;
        let tx = Arc::new(Mutex::new(Some(tx)));
        Ok(Box::new(SqlUnitOfWork {
            tx: tx.clone(),
            book_write_repo: BookWriteRepoTx { tx: tx.clone() },
            book_copy_write_repo: BookCopyWriteRepoTx { tx: tx.clone() },
            membership_write_repo: MemberWriteRepoTx { tx: tx.clone() },
            loan_write_repo: LoanWriteRepoTx { tx: tx.clone() },
        }))
    }
}
