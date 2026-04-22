use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::member::{
    port::MemberWriteRepoPort, Member, MemberId, MemberIdent, MemberPrepared, MemberStatus,
};
use sqlx::{Postgres, Transaction};
use tokio::sync::Mutex;

use crate::member::{member_status_ident, parse_member_status};

#[derive(sqlx::FromRow)]
pub struct MemberPreparedResult {
    pub member_id: i32,
}

#[derive(sqlx::FromRow)]
pub struct MemberUpdatedDbRow {
    pub member_id: i32,
    pub member_ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: String,
    pub full_name: String,
    pub max_active_loans: i16,
}

impl TryFrom<MemberUpdatedDbRow> for Member {
    type Error = anyhow::Error;

    fn try_from(value: MemberUpdatedDbRow) -> Result<Self> {
        Ok(Self {
            id: MemberId(value.member_id),
            ident: MemberIdent(value.member_ident),
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            status: parse_member_status(&value.status)?,
            full_name: value.full_name,
            max_active_loans: value.max_active_loans,
        })
    }
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
            String::from(insert.ident.clone()),
            member_status_ident(&insert.status),
            insert.full_name,
            insert.max_active_loans,
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to create member")?;

        Ok(Member {
            id: MemberId(
                prepared_result
                    .member_id
                    .try_into()
                    .context("member_id exceeds domain range")?,
            ),
            ident: insert.ident.clone(),
            dt_created: Utc::now(),
            dt_modified: Utc::now(),
            status: insert.status.clone(),
            full_name: insert.full_name.clone(),
            max_active_loans: insert.max_active_loans,
        })
    }

    async fn update_status(&self, id: MemberId, status: MemberStatus) -> Result<Member> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().context("Transaction already consumed")?;
        let row = sqlx::query_file_as!(
            MemberUpdatedDbRow,
            "sql/member/commands/update_status.sql",
            i32::from(id.0),
            member_status_ident(&status),
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to update member status")?;

        row.try_into()
    }
}
