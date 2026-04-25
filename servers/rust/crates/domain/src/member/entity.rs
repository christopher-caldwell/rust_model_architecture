use chrono::{DateTime, Utc};

use super::enums::MemberStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MemberId(pub i32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MemberIdent(pub String);

impl From<MemberIdent> for String {
    fn from(value: MemberIdent) -> Self {
        value.0
    }
}

pub struct Member {
    pub id: MemberId,
    pub ident: MemberIdent,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: MemberStatus,
    pub full_name: String,
    pub max_active_loans: i16,
}

pub struct MemberCreationPayload {
    pub full_name: String,
    pub max_active_loans: i16,
}

pub struct MemberPrepared {
    pub ident: MemberIdent,
    pub full_name: String,
    pub max_active_loans: i16,
    pub status: MemberStatus,
}
