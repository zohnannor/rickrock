//! TODO

use tracing_subscriber::EnvFilter;

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
