//! GraphQL API server.

#![expect(unstable_features, reason = "using nightly features")]
#![feature(sync_nonpoison, nonpoison_mutex)]
#![cfg_attr(
    test,
    expect(clippy::missing_panics_doc, reason = "tests are intended to panic")
)]

mod api;
mod domain;
mod infra;
mod service;
mod unused {
    //! Unused dependencies
    // Used in bin
    use dotenvy as _;
    // Not actually used, for `--cfg docsrs` only.
    #[expect(
        deprecated,
        reason = "pinned to 0.14.9 to fix `--cfg docsrs`"
    )]
    use generic_array as _;
    use tracing_subscriber as _;
}

use std::{env, io, sync::Arc};

use axum::{
    Extension, Router,
    routing::{get, post},
};
use derive_more::{Display, Error, From};
use juniper::{EmptySubscription, RootNode};
use juniper_axum::{graphql, playground};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::{
    api::graphql::{Mutation, Query},
    infra::storage::{self, Storage},
    service::Service,
};

/// Error produced by the server.
#[derive(Debug, Error, Display, From)]
#[non_exhaustive]
pub enum ServerError {
    /// I/O error.
    #[display("IO error: {_0}")]
    Io(#[from] io::Error),
    /// Environment variable error.
    #[display("Env error: {_0}")]
    Env(#[from] env::VarError),
    /// Database error.
    #[display("Database error: {_0}")]
    Db(#[from] sqlx::Error),
}

/// GraphQL API schema.
type Schema = RootNode<
    Query<Service<Storage<sqlx::PgPool>>>,
    Mutation<Service<Storage<sqlx::PgPool>>>,
    EmptySubscription,
>;

/// Initializes the dependencies of the server and starts it, listening on
/// `0.0.0.0:3000`.
///
/// # Errors
///
/// If any of the dependencies fails to initialize.
#[inline]
pub async fn setup() -> Result<(), ServerError> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    tracing::info!("Connected to db!");

    let db = Storage::new(pool);

    let service = Service::new(db);

    let schema = Schema::new(
        Query { _service: service.clone() },
        Mutation { service: service.clone() },
        EmptySubscription::new(),
    );

    #[cfg(not(debug_assertions))]
    schema.disable_introspection();

    let router = Router::new()
        .route("/", get(async || "Hello, World!"))
        .route("/graphql", post(graphql::<Arc<Schema>>))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .layer(Extension(Arc::new(schema)));

    tracing::info!("Starting server...");
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}
