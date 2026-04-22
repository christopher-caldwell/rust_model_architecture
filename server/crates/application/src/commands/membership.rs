use anyhow::Context;
use domain::{
    member::{Member, MemberCreationPayload, MemberError, MemberIdent, MemberStatus},
    uow::WriteUnitOfWorkFactory,
};
use std::sync::Arc;

use crate::ports::gen_ident::IdentGeneratorPort;

#[derive(Clone)]
pub struct MembershipCommands {
    uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
    ident_generator: Arc<dyn IdentGeneratorPort>,
}

impl MembershipCommands {
    #[must_use]
    pub fn new(
        uow_factory: Arc<dyn WriteUnitOfWorkFactory>,
        ident_generator: Arc<dyn IdentGeneratorPort>,
    ) -> Self {
        Self {
            uow_factory,
            ident_generator,
        }
    }

    async fn change_member_status(
        &self,
        member: Member,
        new_status: MemberStatus,
        added_context: &'static str,
    ) -> anyhow::Result<Member> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let result = uow
            .membership_write_repo()
            .update_status(member.id, new_status)
            .await
            .context(added_context)?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }

    pub async fn register_member(
        &self,
        payload: MemberCreationPayload,
    ) -> Result<Member, anyhow::Error> {
        let ident = MemberIdent(self.ident_generator.gen());
        let prepared = payload.prepare(ident);
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let result = uow
            .membership_write_repo()
            .create(&prepared)
            .await
            .context("Failed to register member")?;
        uow.commit().await.context("Failed to commit transaction")?;
        Ok(result)
    }

    pub async fn suspend_member(&self, member: Member) -> anyhow::Result<Member> {
        anyhow::ensure!(member.can_be_suspended(), MemberError::CannotBeSuspended);
        self.change_member_status(member, MemberStatus::Suspended, "Failed to suspend member")
            .await
    }

    pub async fn reactivate_member(&self, member: Member) -> anyhow::Result<Member> {
        anyhow::ensure!(
            member.can_be_reactivated(),
            MemberError::CannotBeReactivated
        );
        self.change_member_status(member, MemberStatus::Active, "Failed to reactivate member")
            .await
    }
}
