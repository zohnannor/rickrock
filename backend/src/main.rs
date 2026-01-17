//! TODO

mod db;
mod graphql;
mod unused {
    //! Not actually used, for `--cfg docsrs` only.
    #[expect(
        deprecated,
        reason = "pinned to 0.14.9 to fix `--cfg docsrs`"
    )]
    use generic_array as _;
}

use std::{env, io, sync::Arc};

use axum::{
    Extension, Router,
    routing::{get, post},
};
use derive_more::{Display, Error, From};
use juniper_axum::{graphql, playground};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::graphql::{Mutation, Query, Schema};

/// TODO
#[derive(Debug, Error, Display, From)]
enum Error {
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
///
/// # Panics
///
/// # Errors
#[expect(clippy::single_call_fn, reason = "`setup` is a single-call function")]
async fn setup() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    tracing::info!("Connected to db!");

    let db = db::Db::new(pool);

    let schema = Schema::new(
        Query { db: db.clone() },
        Mutation { db: db.clone() },
        juniper::EmptySubscription::new(),
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

/// TODO
///
/// # Panics
///
/// # Errors
#[tokio::main]
async fn main() {
    drop(dotenvy::dotenv().ok());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::builder().from_env_lossy())
        .with_file(false)
        .compact()
        .init();

    if let Err(err) = setup().await {
        tracing::error!("Error: {err}");
    }
}
