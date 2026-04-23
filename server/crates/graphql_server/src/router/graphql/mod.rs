use anyhow::Error as AnyhowError;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptySubscription, Error, ErrorExtensions, MergedObject, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::State, response::Html};

use server_bootstrap::{CommandError, ServerDeps};

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

pub(crate) fn gql_command_error(error: CommandError) -> Error {
    match error {
        CommandError::NotFound { entity } => {
            Error::new(format!("{entity} not found")).extend_with(|_, e| {
                e.set("code", "NOT_FOUND");
            })
        }
        CommandError::Conflict { message } => Error::new(message).extend_with(|_, e| {
            e.set("code", "CONFLICT");
        }),
        CommandError::Unexpected(error) => {
            tracing::error!("Unhandled GraphQL error: {error:?}");
            Error::new("Something went wrong").extend_with(|_, e| {
                e.set("code", "INTERNAL_SERVER_ERROR");
            })
        }
    }
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
