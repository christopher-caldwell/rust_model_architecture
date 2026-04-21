use std::sync::Arc;

use crate::contact_inquiry::write_repo::ContactInquiryWriteRepoTx;
use application::contact_inquiry::ContactInquiryWriteRepoPort;
use application::uow_port::{UnitOfWorkPort, WriteUnitOfWorkFactory};
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use tokio::sync::Mutex;
use anyhow::Context;

pub struct SqlUnitOfWork {
    tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
    contact_inquiry_write_repo: ContactInquiryWriteRepoTx,
}

#[async_trait]
impl UnitOfWorkPort for SqlUnitOfWork {
    fn contact_inquiry_write_repo(&self) -> &dyn ContactInquiryWriteRepoPort {
        &self.contact_inquiry_write_repo
    }
    async fn commit(self: Box<Self>) -> anyhow::Result<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard
            .take()
            .context("Transaction already consumed")?;
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
            contact_inquiry_write_repo: ContactInquiryWriteRepoTx { tx },
        }))
    }
}
