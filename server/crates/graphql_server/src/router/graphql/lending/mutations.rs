use async_graphql::{Context, Object, Result};

use crate::router::graphql::catalog::InventoryCopy;
use crate::router::graphql::lending::{LoanRecord, StartLoanInput};
use crate::router::graphql::{deps, gql_command_error};
use server_bootstrap::CheckOutBookCopyInput;

#[derive(Default)]
pub struct LendingMutation;

#[Object]
impl LendingMutation {
    async fn check_out_book_copy(
        &self,
        ctx: &Context<'_>,
        input: StartLoanInput,
    ) -> Result<LoanRecord> {
        let deps = deps(ctx);
        let input = CheckOutBookCopyInput {
            member_ident: input.member_number,
            book_copy_barcode: input.barcode,
        };
        let loan = deps
            .lending
            .commands
            .check_out_book_copy(input)
            .await
            .map_err(gql_command_error)?;

        Ok(LoanRecord::from(loan))
    }

    async fn return_book_copy(&self, ctx: &Context<'_>, barcode: String) -> Result<LoanRecord> {
        let deps = deps(ctx);
        let loan = deps
            .lending
            .commands
            .return_book_copy(barcode)
            .await
            .map_err(gql_command_error)?;

        Ok(LoanRecord::from(loan))
    }

    async fn report_lost_loaned_book_copy(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let updated = deps
            .lending
            .commands
            .report_lost_loaned_book_copy(barcode)
            .await
            .map_err(gql_command_error)?;

        Ok(InventoryCopy::from(updated))
    }
}
