use anyhow::Context;
use domain::member::Member;
use std::sync::Arc;

use crate::ports::read_repos::MemberReadRepoPort;

#[derive(Clone)]
pub struct MembershipQueries {
    member_read_repo: Arc<dyn MemberReadRepoPort>,
}

impl MembershipQueries {
    #[must_use]
    pub fn new(
        member_read_repo: Arc<dyn MemberReadRepoPort>,
    ) -> Self {
        Self { member_read_repo }
    }

    pub async fn get_member_details(
        &self,
        member_id: i16,
    ) -> anyhow::Result<Option<Member>> {
        let result = self
            .member_read_repo
            .get_member_details(member_id)
            .await
            .context("Failed to get member details")?;

        Ok(result)
    }
}
