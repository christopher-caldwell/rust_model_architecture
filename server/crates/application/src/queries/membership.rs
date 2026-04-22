use anyhow::Context;
use domain::member::{port::MemberReadRepoPort, Member, MemberIdent};
use std::sync::Arc;

#[derive(Clone)]
pub struct MembershipQueries {
    member_read_repo: Arc<dyn MemberReadRepoPort>,
}

impl MembershipQueries {
    #[must_use]
    pub fn new(member_read_repo: Arc<dyn MemberReadRepoPort>) -> Self {
        Self { member_read_repo }
    }

    pub async fn get_member_details(&self, ident: &MemberIdent) -> anyhow::Result<Option<Member>> {
        self.member_read_repo
            .get_by_ident(ident)
            .await
            .context("Failed to get member details")
    }
}
