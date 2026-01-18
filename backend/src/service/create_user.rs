//! Creation of a [`User`] of a platform by a [`Service`].

use derive_more::{Display, Error, From};

use crate::{
    api::UserService,
    domain::user::{self, User},
    infra::{Repository, Transaction, storage},
    service::Service,
};

impl<R, Tx> UserService for Service<R>
where
    R: Transaction<Error = storage::Error> + Sync,
    Self: Transaction<Error = storage::Error, Transacted = Service<Tx>>,
    Tx: Repository<User, Error = storage::Error> + Send,
{
    type Error = ExecutionError;

    #[inline]
    async fn create_user(&self) -> Result<bool, ExecutionError> {
        let svc = self.tx().await?;
        let user = User { id: user::Id::new() };
        Ok(svc.repo.insert(user).await?)
    }
}

/// Errors produced by [`Service::create_user`].
#[derive(Debug, Display, Error, From, PartialEq, Eq)]
pub enum ExecutionError {
    /// [`Repository`] error.
    #[display("Repository error: {_0}")]
    Repo(#[from] storage::Error),
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::service::repo::mock;

    #[tokio::test]
    async fn creates_user() {
        let service = Service::new(mock::Repo::default());

        let res = service.create_user().await;

        assert_eq!(res, Ok(true), "user should be created");
    }
}

mod graphql {
    //! Conversions between [`domain`] and [`graphql`] types.
    //!
    //! [`domain`]: crate::domain
    //! [`graphql`]: crate::api::graphql

    use super::ExecutionError;
    use crate::api::graphql::Error;

    impl From<ExecutionError> for Error {
        fn from(err: ExecutionError) -> Self {
            Self { message: err.to_string() }
        }
    }
}
