use anyhow::Context;
use chrono::Utc;
use domain::{
    member::{Member, MemberCreationPayload, MemberError, MemberIdent},
    uow::{UnitOfWorkPort, WriteUnitOfWorkFactory},
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

    async fn get_member_by_ident(
        &self,
        uow: &dyn UnitOfWorkPort,
        member_ident: &str,
    ) -> Result<Member, super::CommandError> {
        let ident = MemberIdent(member_ident.to_owned());
        uow.membership_write_repo()
            .get_by_ident_for_update(&ident)
            .await
            .context("Failed to load member for write")?
            .ok_or(MemberError::NotFound.into())
    }

    pub async fn register_member(
        &self,
        payload: MemberCreationPayload,
    ) -> Result<Member, super::CommandError> {
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

    pub async fn suspend_member(
        &self,
        input: super::MemberIdentInput,
    ) -> Result<Member, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let member = self.get_member_by_ident(&*uow, &input.member_ident).await?;
        let suspended_status = member.suspend()?;
        uow.membership_write_repo()
            .update_status(member.id, suspended_status.clone())
            .await
            .context("Failed to suspend member")?;
        uow.commit().await.context("Failed to commit transaction")?;
        let updated_member = Member {
            status: suspended_status,
            dt_modified: Utc::now(),
            ..member
        };
        Ok(updated_member)
    }

    pub async fn reactivate_member(
        &self,
        input: super::MemberIdentInput,
    ) -> Result<Member, super::CommandError> {
        let uow = self
            .uow_factory
            .build()
            .await
            .context("Failed to build unit of work")?;
        let member = self.get_member_by_ident(&*uow, &input.member_ident).await?;
        let active_status = member.reactivate()?;
        uow.membership_write_repo()
            .update_status(member.id, active_status.clone())
            .await
            .context("Failed to reactivate member")?;
        uow.commit().await.context("Failed to commit transaction")?;
        let updated_member = Member {
            status: active_status,
            dt_modified: Utc::now(),
            ..member
        };
        Ok(updated_member)
    }
}
