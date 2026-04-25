use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::Utc;
use domain::member::{
    port::MemberWriteRepoPort, Member, MemberId, MemberIdent, MemberPrepared, MemberStatus,
};
use sqlx::{Postgres, Transaction};
use tokio::sync::Mutex;

use crate::member::read_repo::MemberDbRow;

#[derive(sqlx::FromRow)]
pub struct MemberPreparedResult {
    pub member_id: i32,
}

pub struct MemberWriteRepoTx {
    pub tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

#[async_trait]
impl MemberWriteRepoPort for MemberWriteRepoTx {
    async fn create(&self, insert: &MemberPrepared) -> Result<Member> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let prepared_result = sqlx::query_file_as!(
            MemberPreparedResult,
            "sql/member/commands/create.sql",
            insert.ident.0.clone(),
            insert.status.to_string(),
            insert.full_name,
            insert.max_active_loans,
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to create member")?;

        let now = Utc::now();
        let created_member = Member {
            id: MemberId(prepared_result.member_id),
            ident: insert.ident.clone(),
            dt_created: now,
            dt_modified: now,
            status: insert.status.clone(),
            full_name: insert.full_name.clone(),
            max_active_loans: insert.max_active_loans,
        };
        Ok(created_member)
    }

    async fn get_by_ident_for_update(&self, ident: &MemberIdent) -> Result<Option<Member>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let ident_str = ident.0.clone();
        let row = sqlx::query_file_as!(
            MemberDbRow,
            "sql/member/commands/get_by_ident_for_update.sql",
            ident_str
        )
        .fetch_optional(&mut **tx)
        .await
        .context("Failed to fetch member by ident")?;

        row.map(Member::try_from).transpose()
    }

    async fn update_status(&self, id: MemberId, status: MemberStatus) -> Result<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        sqlx::query_file!(
            "sql/member/commands/update_status.sql",
            id.0,
            status.to_string(),
        )
        .execute(&mut **tx)
        .await
        .context("Failed to update member status")?;

        Ok(())
    }
}
