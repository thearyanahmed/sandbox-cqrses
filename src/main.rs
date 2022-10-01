use std::path::Path;
use std::sync::Arc;
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use crate::domain::aggregate::BankAccount;

mod domain;
mod services;

#[tokio::main]
async fn main() {
    let pool = default_postgress_pool("postgresql://demo_user:demo_pass@localhost:5432/demo").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    // Configure the Axum routes and services.
    // For this example a single logical endpoint is used and the HTTP method
    // distinguishes whether the call is a command or a query.
    let router = Router::new()
        .route(
            "/account/:account_id",
            get(query_handler).post(command_handler),
        )
        .layer(Extension(cqrs))
        .layer(Extension(account_query));

    // Start the Axum server.
    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn query_handler(
    Path(account_id): Box<Path>,
    Extension(view_repo): Extension<Arc<PostgresViewRepository<BankAccountView, BankAccount>>>,
) -> Response {
    let view = match view_repo {

    };
}

async fn command_handler() -> Response {

}