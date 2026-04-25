use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::member::{port::MemberReadRepoPort, Member, MemberId, MemberIdent, MemberStatus};
use sqlx::PgPool;

use std::str::FromStr;

#[derive(sqlx::FromRow)]
pub struct MemberDbRow {
    pub member_id: i32,
    pub member_ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: String,
    pub full_name: String,
    pub max_active_loans: i16,
}

impl TryFrom<MemberDbRow> for Member {
    type Error = anyhow::Error;

    fn try_from(value: MemberDbRow) -> Result<Self> {
        Ok(Self {
            id: MemberId(value.member_id),
            ident: MemberIdent(value.member_ident),
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            status: MemberStatus::from_str(&value.status).context("Invalid member status in DB")?,
            full_name: value.full_name,
            max_active_loans: value.max_active_loans,
        })
    }
}

pub struct MemberReadRepoSql {
    pub pool: PgPool,
}

#[async_trait]
impl MemberReadRepoPort for MemberReadRepoSql {
    async fn get_by_id(&self, member_id: MemberId) -> Result<Option<Member>> {
        let row =
            sqlx::query_file_as!(MemberDbRow, "sql/member/queries/get_by_id.sql", member_id.0)
                .fetch_optional(&self.pool)
                .await
                .context("Failed to fetch member by id")?;

        row.map(Member::try_from).transpose()
    }

    async fn get_by_ident(&self, ident: &MemberIdent) -> Result<Option<Member>> {
        let ident_str: String = ident.clone().into();
        let row = sqlx::query_file_as!(
            MemberDbRow,
            "sql/member/queries/get_by_ident.sql",
            ident_str
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch member by ident")?;

        row.map(Member::try_from).transpose()
    }
}
