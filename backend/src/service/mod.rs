//! GraphQL API service.

use crate::infra::{Transaction, storage};

mod create_user;
mod repo;

/// GraphQL API service.
#[derive(Debug, Clone)]
pub struct Service<R> {
    /// [`Repository`] of this [`Service`].
    ///
    /// [`Repository`]: crate::infra::Repository
    repo: R,
}

impl<R> Service<R> {
    /// Creates a new [`Service`] with the provided [`Repository`].
    ///
    /// [`Repository`]: crate::infra::Repository
    pub(crate) const fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R> Transaction for Service<R>
where
    R: Transaction<Error = storage::Error> + Sync,
{
    type Transacted = Service<R::Transacted>;
    type Error = storage::Error;

    async fn tx(&self) -> Result<Self::Transacted, Self::Error> {
        Ok(Service { repo: self.repo.tx().await? })
    }
}
