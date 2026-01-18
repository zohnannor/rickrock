//! Persistent storage of a [`Service`].
//!
//! [`Service`]: crate::service::Service

use std::mem;

use derive_more::{Display, Error, From};

use crate::infra::Transaction;

#[expect(
    clippy::error_impl_error,
    reason = "intended to be used as `db::Error`"
)]
/// Error produced by [`Storage`].
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[expect(clippy::doc_markdown, reason = "false-positive")]
    /// PostgreSQL error.
    #[display("PostgreSQL error: {_0}")]
    DbError(#[from] sqlx::Error),
}

impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DbError(l0), Self::DbError(r0)) => {
                mem::discriminant(l0) == mem::discriminant(r0)
            }
        }
    }
}

/// Persistent storage of a [`Service`].
///
/// [`Service`]: crate::service::Service
#[derive(Debug, Clone)]
pub struct Storage<DB> {
    /// Underlying database storage.
    db: DB,
}

impl<DB> Storage<DB> {
    /// Creates a new [`Storage`] from the provided `db`.
    #[expect(
        clippy::single_call_fn,
        reason = "storage typically only created once at the server \
                  initialization"
    )]
    pub(crate) const fn new(db: DB) -> Self {
        Self { db }
    }
}

impl Transaction for Storage<sqlx::PgPool> {
    type Transacted = Storage<sqlx::PgTransaction<'static>>;
    type Error = Error;

    async fn tx(&self) -> Result<Self::Transacted, Self::Error> {
        let tx = self.db.begin().await?;
        Ok(Storage { db: tx })
    }
}
