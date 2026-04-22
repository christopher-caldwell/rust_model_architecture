use async_trait::async_trait;

use super::{Member, MemberId, MemberPrepared, MemberStatus};

#[async_trait]
pub trait MemberWriteRepoPort: Send + Sync {
    async fn create(&self, insert: &MemberPrepared) -> anyhow::Result<Member>;
    async fn update_status(&self, id: MemberId, status: MemberStatus) -> anyhow::Result<Member>;
}

#[async_trait]
pub trait MemberReadRepoPort: Send + Sync {
    async fn get_by_id(&self, id: MemberId) -> anyhow::Result<Option<Member>>;
}
