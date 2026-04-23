use async_graphql::{Context, Object, Result};

use crate::router::graphql::catalog::InventoryCopy;
use crate::router::graphql::lending::{LoanRecord, StartLoanInput};
use crate::router::graphql::{deps, find_copy, find_member, gql_service_error};

#[derive(Default)]
pub struct LendingMutation;

#[Object]
impl LendingMutation {
    async fn start_loan(&self, ctx: &Context<'_>, input: StartLoanInput) -> Result<LoanRecord> {
        let deps = deps(ctx);
        let member = find_member(deps, input.member_number).await?;
        let copy = find_copy(deps, input.barcode).await?;
        let loan = deps
            .lending
            .commands
            .check_out_book_copy(member, copy)
            .await
            .map_err(gql_service_error)?;

        Ok(LoanRecord::from(loan))
    }

    async fn check_in_copy(&self, ctx: &Context<'_>, barcode: String) -> Result<LoanRecord> {
        let deps = deps(ctx);
        let copy = find_copy(deps, barcode).await?;
        let loan = deps
            .lending
            .commands
            .return_book_copy(copy)
            .await
            .map_err(gql_service_error)?;

        Ok(LoanRecord::from(loan))
    }

    async fn close_loan_as_lost(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let copy = find_copy(deps, barcode).await?;
        let updated = deps
            .lending
            .commands
            .report_lost_loaned_book_copy(copy)
            .await
            .map_err(gql_service_error)?;

        Ok(InventoryCopy::from(updated))
    }
}
