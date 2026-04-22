use chrono::{DateTime, Utc};
use domain::member::{Member, MemberCreationPayload, MemberStatus};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const MEMBERS_TAG: &str = "Members";
pub const MEMBERS_PATH: &str = "/members";
pub const MEMBER_BY_ID_PATH: &str = "/members/{ident}";
pub const MEMBER_SUSPENSION_PATH: &str = "/members/{ident}/suspension";
pub const MEMBER_LOANS_PATH: &str = "/members/{ident}/loans";

#[derive(Serialize, ToSchema)]
pub struct MemberResponseBody {
    pub ident: String,
    pub dt_created: DateTime<Utc>,
    pub dt_modified: DateTime<Utc>,
    pub status: String,
    pub full_name: String,
    pub max_active_loans: i16,
}

impl From<Member> for MemberResponseBody {
    fn from(value: Member) -> Self {
        Self {
            ident: value.ident.into(),
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            status: member_status_text(&value.status),
            full_name: value.full_name,
            max_active_loans: value.max_active_loans,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateMemberRequestBody {
    pub full_name: String,
    pub max_active_loans: i16,
}

impl From<CreateMemberRequestBody> for MemberCreationPayload {
    fn from(value: CreateMemberRequestBody) -> Self {
        Self {
            full_name: value.full_name,
            max_active_loans: value.max_active_loans,
        }
    }
}

fn member_status_text(status: &MemberStatus) -> String {
    match status {
        MemberStatus::Active => String::from("active"),
        MemberStatus::Suspended => String::from("suspended"),
    }
}
