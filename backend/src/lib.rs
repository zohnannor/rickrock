//! TODO

mod db;
mod graphql;
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
mod user;

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
    graphql::{Mutation, Query},
    service::Service,
};

/// TODO
#[derive(Debug, Error, Display, From)]
#[non_exhaustive]
pub enum ServerError {
    /// TODO
    #[display("IO error: {_0}")]
    Io(#[from] io::Error),
    /// TODO
    #[display("Env error: {_0}")]
    Env(#[from] env::VarError),
    /// TODO
    #[display("Database error: {_0}")]
    Db(#[from] sqlx::Error),
}

/// TODO
type Schema = RootNode<
    Query<Service<db::Db>>,
    Mutation<Service<db::Db>>,
    EmptySubscription,
>;

/// TODO
///
/// # Panics
///
/// # Errors
#[inline]
pub async fn setup() -> Result<(), ServerError> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    tracing::info!("Connected to db!");

    let db = db::Db::new(pool);

    let service = Service::new(db);

    let schema = Schema::new(
        Query { service: service.clone() },
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
