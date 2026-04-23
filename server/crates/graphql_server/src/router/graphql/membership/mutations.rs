use async_graphql::{Context, Object, Result};

use crate::router::graphql::membership::{LibraryMember, RegisterMemberInput};
use crate::router::graphql::{deps, find_member, gql_service_error};

#[derive(Default)]
pub struct MembershipMutation;

#[Object]
impl MembershipMutation {
    async fn register_member(
        &self,
        ctx: &Context<'_>,
        input: RegisterMemberInput,
    ) -> Result<LibraryMember> {
        let deps = deps(ctx);
        let member = deps
            .membership
            .commands
            .register_member(input.into())
            .await
            .map_err(gql_service_error)?;

        Ok(LibraryMember::from(member))
    }

    async fn suspend_member(
        &self,
        ctx: &Context<'_>,
        member_number: String,
    ) -> Result<LibraryMember> {
        let deps = deps(ctx);
        let member = find_member(deps, member_number).await?;
        let updated = deps
            .membership
            .commands
            .suspend_member(member)
            .await
            .map_err(gql_service_error)?;

        Ok(LibraryMember::from(updated))
    }

    async fn reactivate_member(
        &self,
        ctx: &Context<'_>,
        member_number: String,
    ) -> Result<LibraryMember> {
        let deps = deps(ctx);
        let member = find_member(deps, member_number).await?;
        let updated = deps
            .membership
            .commands
            .reactivate_member(member)
            .await
            .map_err(gql_service_error)?;

        Ok(LibraryMember::from(updated))
    }
}
