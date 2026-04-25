use async_graphql::{Context, Object, Result};

use crate::router::graphql::membership::{LibraryMember, RegisterMemberInput};
use crate::router::graphql::{deps, gql_command_error};
use server_bootstrap::MemberIdentInput;

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
            .map_err(gql_command_error)?;

        Ok(LibraryMember::from(member))
    }

    async fn suspend_member(
        &self,
        ctx: &Context<'_>,
        member_number: String,
    ) -> Result<LibraryMember> {
        let deps = deps(ctx);
        let input = MemberIdentInput {
            member_ident: member_number,
        };
        let updated = deps
            .membership
            .commands
            .suspend_member(input)
            .await
            .map_err(gql_command_error)?;

        Ok(LibraryMember::from(updated))
    }

    async fn reactivate_member(
        &self,
        ctx: &Context<'_>,
        member_number: String,
    ) -> Result<LibraryMember> {
        let deps = deps(ctx);
        let input = MemberIdentInput {
            member_ident: member_number,
        };
        let updated = deps
            .membership
            .commands
            .reactivate_member(input)
            .await
            .map_err(gql_command_error)?;

        Ok(LibraryMember::from(updated))
    }
}
