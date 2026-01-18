//! TODO

use tracing_subscriber::EnvFilter;

mod unused {
    //! Used in lib
    use axum as _;
    use derive_more as _;
    // Not actually used, for `--cfg docsrs` only.
    #[expect(
        deprecated,
        reason = "pinned to 0.14.9 to fix `--cfg docsrs`"
    )]
    use generic_array as _;
    use juniper as _;
    use juniper_axum as _;
    #[cfg(test)]
    use pretty_assertions as _;
    use sqlx as _;
    use uuid as _;
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

    if let Err(err) = backend::setup().await {
        tracing::error!("Error: {err}");
    }
}
