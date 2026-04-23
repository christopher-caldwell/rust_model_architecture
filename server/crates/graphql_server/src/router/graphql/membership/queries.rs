use async_graphql::{Context, Object, Result};
use domain::member::MemberIdent;

use crate::router::graphql::membership::LibraryMember;
use crate::router::graphql::{deps, gql_service_error};

#[derive(Default)]
pub struct MembershipQuery;

#[Object]
impl MembershipQuery {
    async fn member_account(
        &self,
        ctx: &Context<'_>,
        member_number: String,
    ) -> Result<Option<LibraryMember>> {
        let deps = deps(ctx);
        let member = deps
            .membership
            .queries
            .get_member_details(&MemberIdent(member_number))
            .await
            .map_err(gql_service_error)?;

        Ok(member.map(LibraryMember::from))
    }
}
