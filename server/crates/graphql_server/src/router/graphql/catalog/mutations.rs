use async_graphql::{Context, Object, Result};
use domain::book_copy::BookCopyCreationPayload;

use crate::router::graphql::catalog::{
    AddInventoryCopyInput, CatalogTitle, CreateCatalogTitleInput, InventoryCopy,
};
use crate::router::graphql::{deps, find_copy, gql_not_found, gql_service_error};

#[derive(Default)]
pub struct CatalogMutation;

#[Object]
impl CatalogMutation {
    async fn create_catalog_title(
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
            .map_err(gql_service_error)?;

        Ok(CatalogTitle::from(book))
    }

    async fn add_inventory_copy(
        &self,
        ctx: &Context<'_>,
        input: AddInventoryCopyInput,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let book = deps
            .catalog
            .queries
            .get_book_by_isbn(&input.isbn)
            .await
            .map_err(gql_service_error)?
            .ok_or_else(|| gql_not_found("Book"))?;

        let copy = deps
            .catalog
            .commands
            .add_book_copy(BookCopyCreationPayload {
                barcode: input.barcode,
                author_name: input.author_name,
                book_id: book.id,
            })
            .await
            .map_err(gql_service_error)?;

        Ok(InventoryCopy::from(copy))
    }

    async fn declare_copy_lost(&self, ctx: &Context<'_>, barcode: String) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let copy = find_copy(deps, barcode).await?;
        let updated = deps
            .catalog
            .commands
            .mark_book_copy_lost(copy)
            .await
            .map_err(gql_service_error)?;

        Ok(InventoryCopy::from(updated))
    }

    async fn restore_lost_copy(&self, ctx: &Context<'_>, barcode: String) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let copy = find_copy(deps, barcode).await?;
        let updated = deps
            .catalog
            .commands
            .mark_book_copy_found(copy)
            .await
            .map_err(gql_service_error)?;

        Ok(InventoryCopy::from(updated))
    }

    async fn queue_copy_for_maintenance(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let copy = find_copy(deps, barcode).await?;
        let updated = deps
            .catalog
            .commands
            .send_book_copy_to_maintenance(copy)
            .await
            .map_err(gql_service_error)?;

        Ok(InventoryCopy::from(updated))
    }

    async fn finish_copy_maintenance(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> Result<InventoryCopy> {
        let deps = deps(ctx);
        let copy = find_copy(deps, barcode).await?;
        let updated = deps
            .catalog
            .commands
            .complete_book_copy_maintenance(copy)
            .await
            .map_err(gql_service_error)?;

        Ok(InventoryCopy::from(updated))
    }
}
