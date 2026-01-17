//! TODO

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
use juniper::{EmptySubscription, RootNode, graphql_object};
use juniper_axum::{graphql, playground};
use sqlx::{Connection as _, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

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
#[derive(Debug, Default)]
struct Query;

#[graphql_object]
impl Query {
    /// TODO
    fn hello_world() -> String {
        "Hello, World!".to_owned()
    }
}

/// TODO
#[derive(Debug, Default)]

struct Mutation;

#[graphql_object]
impl Mutation {
    /// TODO
    fn hello_world() -> String {
        "Hello, World!".to_owned()
    }
}

/// TODO
type Schema = RootNode<Query, Mutation, EmptySubscription>;

/// TODO
struct Repository {
    /// TODO
    pool: sqlx::PgPool,
}

impl Repository {
    /// TODO
    #[expect(clippy::single_call_fn, reason = "TODO")]
    const fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
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

    let repo = Repository::new(pool);

    {
        let mut conn = repo.pool.acquire().await?;
        let mut tx = conn.begin().await?;

        let result = sqlx::query!(r#"SELECT 1 + 1 AS "answer!""#)
            .fetch_one(&mut *tx)
            .await?;
        tracing::info!("Answer: {}", result.answer);
        tx.commit().await?;
    }

    let schema = Schema::new(Query, Mutation, EmptySubscription::new());

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
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_file(false)
        .compact()
        .init();

    if let Err(err) = setup().await {
        tracing::error!("Error: {err}");
    }
}
