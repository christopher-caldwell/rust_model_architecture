use async_graphql::{Context, Object, Result};

use crate::router::graphql::catalog::{
    AddInventoryCopyInput, CatalogTitle, CreateCatalogTitleInput, InventoryCopy,
};
use crate::router::graphql::{deps, gql_command_error};
use server_bootstrap::AddBookCopyInput;

#[derive(Default)]
pub struct CatalogMutation;

#[Object]
impl CatalogMutation {
    async fn create_book(
        &self,
        ctx: &Context<'_>,
        input: CreateCatalogTitleInput,
    ) -> Result<CatalogTitle> {
        let deps = deps(ctx);
        let book = deps
            .catalog
            .commands
            .add_book(input.into())
            .await
            .map_err(gql_command_error)?;

        Ok(CatalogTitle::from(book))
    }

    async fn add_book_copy(
        &self,
        ctx: &Context<'_>,
        input: AddInventoryCopyInput,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let input = AddBookCopyInput {
            isbn: input.isbn,
            barcode: input.barcode,
        };
        let copy = deps
            .catalog
            .commands
            .add_book_copy(input)
            .await
            .map_err(gql_command_error)?;

        Ok(InventoryCopy::from(copy))
    }

    async fn mark_book_copy_lost(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let updated = deps
            .catalog
            .commands
            .mark_book_copy_lost(barcode)
            .await
            .map_err(gql_command_error)?;

        Ok(InventoryCopy::from(updated))
    }

    async fn mark_book_copy_found(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let updated = deps
            .catalog
            .commands
            .mark_book_copy_found(barcode)
            .await
            .map_err(gql_command_error)?;

        Ok(InventoryCopy::from(updated))
    }

    async fn send_book_copy_to_maintenance(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let updated = deps
            .catalog
            .commands
            .send_book_copy_to_maintenance(barcode)
            .await
            .map_err(gql_command_error)?;

        Ok(InventoryCopy::from(updated))
    }

    async fn complete_book_copy_maintenance(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let updated = deps
            .catalog
            .commands
            .complete_book_copy_maintenance(barcode)
            .await
            .map_err(gql_command_error)?;

        Ok(InventoryCopy::from(updated))
    }
}
