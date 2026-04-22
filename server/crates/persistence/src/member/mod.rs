pub(crate) mod read_repo;
pub(crate) mod write_repo;

use anyhow::{bail, Result};
use domain::member::MemberStatus;

pub use read_repo::MemberReadRepoSql;

pub(crate) fn parse_member_status(status: &str) -> Result<MemberStatus> {
    match status {
        "active" => Ok(MemberStatus::Active),
        "suspended" => Ok(MemberStatus::Suspended),
        _ => bail!("Unknown member status '{status}'"),
    }
}

pub(crate) fn member_status_ident(status: &MemberStatus) -> &'static str {
    match status {
        MemberStatus::Active => "active",
        MemberStatus::Suspended => "suspended",
    }
}
