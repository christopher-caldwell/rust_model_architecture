use async_graphql::{Context, Object, Result};

use crate::router::graphql::catalog::{CatalogTitle, InventoryCopy};
use crate::router::graphql::{deps, gql_service_error};

#[derive(Default)]
pub struct CatalogQuery;

#[Object]
impl CatalogQuery {
    async fn catalog_titles(&self, ctx: &Context<'_>) -> Result<Vec<CatalogTitle>> {
        let deps = deps(ctx);
        let books = deps
            .catalog
            .queries
            .get_book_catalog()
            .await
            .map_err(gql_service_error)?;

        Ok(books.into_iter().map(CatalogTitle::from).collect())
    }

    async fn catalog_title(&self, ctx: &Context<'_>, isbn: String) -> Result<Option<CatalogTitle>> {
        let deps = deps(ctx);
        let book = deps
            .catalog
            .queries
            .get_book_by_isbn(&isbn)
            .await
            .map_err(gql_service_error)?;

        Ok(book.map(CatalogTitle::from))
    }

    async fn inventory_copy(
        &self,
        ctx: &Context<'_>,
        barcode: String,
    ) -> Result<Option<InventoryCopy>> {
        let deps = deps(ctx);
        let copy = deps
            .catalog
            .queries
            .get_book_copy_details(&barcode)
            .await
            .map_err(gql_service_error)?;

        Ok(copy.map(InventoryCopy::from))
    }
}
