//! Creation of a [`User`] of a platform by a [`Service`].

use derive_more::{Display, Error, From};

use crate::{
    domain::{User, user},
    infra::{Repository, storage},
    service::{
        Service,
        command::{Command, Execute, Register, handler::ExecuteCommand},
    },
};

/// [`Command`] for creating a new [`User`].
#[derive(Debug)]
pub struct CreateUser;

impl Command<User> for CreateUser {}

impl<R> Register<CreateUser> for Service<R> {
    type Pipeline = ExecuteCommand<User, R>;
}

impl<R> Execute<User, R> for CreateUser
where
    R: Repository<User, Error = storage::Error> + Sync,
{
    type Output = bool;
    type Error = ExecutionError;

    async fn execute(&self, repo: &R) -> Result<Self::Output, Self::Error> {
        Ok(repo.insert(User { id: user::Id::new() }).await?)
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
    use crate::service::command::Executor as _;

    #[tokio::test]
    async fn creates_user() {
        let service = Service::mock();

        let res: Result<bool, ExecutionError> = service.run(CreateUser).await;

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
