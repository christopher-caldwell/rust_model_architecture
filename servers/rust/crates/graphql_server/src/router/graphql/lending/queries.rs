use async_graphql::{Context, Object, Result};

use crate::router::graphql::lending::LoanRecord;
use crate::router::graphql::{deps, gql_service_error};

#[derive(Default)]
pub struct LendingQuery;

#[Object]
impl LendingQuery {
    async fn member_loans(
        &self,
        ctx: &Context<'_>,
        member_number: String,
    ) -> Result<Vec<LoanRecord>> {
        let deps = deps(ctx);
        let loans = deps
            .lending
            .queries
            .get_member_loans(&domain::member::MemberIdent(member_number))
            .await
            .map_err(gql_service_error)?;

        Ok(loans.into_iter().map(LoanRecord::from).collect())
    }

    async fn overdue_loans(&self, ctx: &Context<'_>) -> Result<Vec<LoanRecord>> {
        let deps = deps(ctx);
        let loans = deps
            .lending
            .queries
            .get_overdue_loans()
            .await
            .map_err(gql_service_error)?;

        Ok(loans.into_iter().map(LoanRecord::from).collect())
    }
}
