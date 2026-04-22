use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::member::{port::MemberReadRepoPort, Member, MemberId};
use sqlx::PgPool;

use crate::member::parse_member_status;

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
            id: MemberId(
                value
                    .member_id
                    .try_into()
                    .context("member_id exceeds domain range")?,
            ),
            ident: value.member_ident,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            status: parse_member_status(&value.status)?,
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
    async fn get_by_id(&self, id: MemberId) -> Result<Option<Member>> {
        let row = sqlx::query_file_as!(
            MemberDbRow,
            "sql/member/queries/get_by_id.sql",
            i32::from(id.0)
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch member by id")?;

        row.map(Member::try_from).transpose()
    }
}
