use anyhow::Error as AnyhowError;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptySubscription, Error, ErrorExtensions, MergedObject, Result, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, response::Html};
use domain::{book_copy::BookCopy, member::Member};

use crate::router::dependencies::ServerDeps;

pub mod catalog;
pub mod lending;
pub mod membership;

pub const GRAPHQL_PATH: &str = "/graphql";

pub type LibrarySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    catalog::CatalogQuery,
    lending::LendingQuery,
    membership::MembershipQuery,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    catalog::CatalogMutation,
    lending::LendingMutation,
    membership::MembershipMutation,
);

pub fn build_schema(deps: ServerDeps) -> LibrarySchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(deps)
    .finish()
}

pub async fn graphql_handler(
    State(schema): State<LibrarySchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

pub async fn graphql_playground() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        GRAPHQL_PATH,
    )))
}

pub(crate) fn deps<'a>(ctx: &'a Context<'a>) -> &'a ServerDeps {
    ctx.data_unchecked::<ServerDeps>()
}

pub(crate) async fn find_member(deps: &ServerDeps, member_number: String) -> Result<Member> {
    deps.membership
        .queries
        .get_member_details(&domain::member::MemberIdent(member_number))
        .await
        .map_err(gql_service_error)?
        .ok_or_else(|| gql_not_found("Member"))
}

pub(crate) async fn find_copy(deps: &ServerDeps, barcode: String) -> Result<BookCopy> {
    deps.catalog
        .queries
        .get_book_copy_details(&barcode)
        .await
        .map_err(gql_service_error)?
        .ok_or_else(|| gql_not_found("Book copy"))
}

pub(crate) fn gql_not_found(entity: &str) -> Error {
    Error::new(format!("{entity} not found")).extend_with(|_, e| {
        e.set("code", "NOT_FOUND");
    })
}

pub(crate) fn gql_service_error(error: AnyhowError) -> Error {
    if let Some(message) = conflict_message(&error) {
        return Error::new(message).extend_with(|_, e| {
            e.set("code", "CONFLICT");
        });
    }

    tracing::error!("Unhandled GraphQL error: {error:?}");
    Error::new("Something went wrong").extend_with(|_, e| {
        e.set("code", "INTERNAL_SERVER_ERROR");
    })
}

fn conflict_message(error: &AnyhowError) -> Option<String> {
    for cause in error.chain() {
        if let Some(inner) = cause.downcast_ref::<domain::book_copy::BookCopyError>() {
            return Some(inner.to_string());
        }
        if let Some(inner) = cause.downcast_ref::<domain::loan::LoanError>() {
            return Some(inner.to_string());
        }
        if let Some(inner) = cause.downcast_ref::<domain::member::MemberError>() {
            return Some(inner.to_string());
        }
    }

    None
}
