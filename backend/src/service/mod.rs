//! GraphQL API service.

pub mod command;
mod repo;

use crate::{
    infra::{
        Transaction,
        storage::{self, Storage},
    },
    service::command::{Command, Executor, Handle, Register, ServiceWrapper},
};

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
    #[expect(clippy::single_call_fn, reason = "constructor")]
    pub(crate) const fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R, Ent, Cmd, H> Executor<Ent, Cmd> for Service<R>
where
    R: Sync,
    Cmd: Command<Ent> + Send,
    Self: Register<Cmd, Pipeline = H>,
    H: for<'svc> From<ServiceWrapper<&'svc Self>> + Handle<Cmd> + Send,
{
    type Output = H::Output;
    type Error = H::Error;

    async fn run(&self, cmd: Cmd) -> Result<Self::Output, Self::Error> {
        H::from(ServiceWrapper(self)).handle(cmd).await
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

impl<R> From<&Self> for Service<R>
where
    Self: Clone,
{
    fn from(value: &Self) -> Self {
        value.clone()
    }
}

impl<DB> From<&Service<Self>> for Storage<DB>
where
    Self: Clone,
{
    fn from(svc: &Service<Self>) -> Self {
        svc.repo.clone()
    }
}

#[cfg(test)]
mod mock {
    use super::*;
    use crate::service::repo::mock;

    impl Service<mock::Repo> {
        #[expect(clippy::single_call_fn, reason = "constructor")]
        pub fn mock() -> Self {
            Self { repo: mock::Repo::default() }
        }
    }

    impl From<&Service<Self>> for mock::Repo
    where
        Self: Clone,
    {
        fn from(svc: &Service<Self>) -> Self {
            svc.repo.clone()
        }
    }
}
