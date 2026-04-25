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
    tracing::error!("Unhandled GraphQL error: {error:?}");
    Error::new("Something went wrong").extend_with(|_, e| {
        e.set("code", "INTERNAL_SERVER_ERROR");
    })
}

pub(crate) fn gql_command_error(error: CommandError) -> Error {
    match error {
        CommandError::Member(e) => gql_member_error(e),
        CommandError::BookCopy(e) => gql_book_copy_error(e),
        CommandError::Book(e) => gql_book_error(e),
        CommandError::Loan(e) => gql_loan_error(e),
        CommandError::Unexpected(e) => {
            tracing::error!("Unhandled GraphQL error: {e:?}");
            Error::new("Something went wrong").extend_with(|_, e| {
                e.set("code", "INTERNAL_SERVER_ERROR");
            })
        }
    }
}

fn gql_not_found(message: impl Into<String>) -> Error {
    Error::new(message.into()).extend_with(|_, e| {
        e.set("code", "NOT_FOUND");
    })
}

fn gql_conflict(message: impl Into<String>) -> Error {
    Error::new(message.into()).extend_with(|_, e| {
        e.set("code", "CONFLICT");
    })
}

fn gql_member_error(e: domain::member::MemberError) -> Error {
    use domain::member::MemberError;
    match e {
        MemberError::NotFound => gql_not_found("Member not found"),
        MemberError::CannotBeSuspended
        | MemberError::CannotBeReactivated
        | MemberError::CannotBorrowWhileSuspended
        | MemberError::LoanLimitReached => gql_conflict(e.to_string()),
    }
}

fn gql_book_copy_error(e: domain::book_copy::BookCopyError) -> Error {
    use domain::book_copy::BookCopyError;
    match e {
        BookCopyError::NotFound => gql_not_found("Book copy not found"),
        BookCopyError::CannotBeBorrowed
        | BookCopyError::CannotBeSentToMaintenance
        | BookCopyError::CannotBeReturnedFromMaintenance
        | BookCopyError::CannotMarkBookLost
        | BookCopyError::CannotBeReturnedFromLost => gql_conflict(e.to_string()),
    }
}

fn gql_book_error(e: domain::book::BookError) -> Error {
    use domain::book::BookError;
    match e {
        BookError::NotFound => gql_not_found("Book not found"),
    }
}

fn gql_loan_error(e: domain::loan::LoanError) -> Error {
    use domain::loan::LoanError;
    match e {
        LoanError::NoActiveLoanForBookCopy | LoanError::CannotBeReturned => {
            gql_conflict(e.to_string())
        }
    }
}
