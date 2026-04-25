use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use domain::member::{Member, MemberCreationPayload};

pub mod mutations;
pub mod queries;

pub use mutations::MembershipMutation;
pub use queries::MembershipQuery;

#[derive(SimpleObject)]
pub struct LibraryMember {
    member_number: String,
    dt_created: DateTime<Utc>,
    dt_modified: DateTime<Utc>,
    status: LibraryMemberStatus,
    full_name: String,
    max_active_loans: i16,
}

impl From<Member> for LibraryMember {
    fn from(value: Member) -> Self {
        Self {
            member_number: value.ident.0,
            dt_created: value.dt_created,
            dt_modified: value.dt_modified,
            status: LibraryMemberStatus::from(value.status),
            full_name: value.full_name,
            max_active_loans: value.max_active_loans,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum LibraryMemberStatus {
    Active,
    Suspended,
}

impl From<domain::member::MemberStatus> for LibraryMemberStatus {
    fn from(value: domain::member::MemberStatus) -> Self {
        match value {
            domain::member::MemberStatus::Active => Self::Active,
            domain::member::MemberStatus::Suspended => Self::Suspended,
        }
    }
}

#[derive(InputObject)]
pub struct RegisterMemberInput {
    full_name: String,
    max_active_loans: i16,
}

impl From<RegisterMemberInput> for MemberCreationPayload {
    fn from(value: RegisterMemberInput) -> Self {
        Self {
            full_name: value.full_name,
            max_active_loans: value.max_active_loans,
        }
    }
}
