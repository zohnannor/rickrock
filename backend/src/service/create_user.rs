//! TODO

use derive_more::{Display, Error, From};

use crate::{
    api::UserService,
    domain::user::User,
    infra::db::{self, Repository},
    service::Service,
};

impl<R> UserService for Service<R>
where
    R: Repository<User, Error = db::Error> + Sync,
{
    type Error = ExecutionError;

    #[inline]
    async fn create_user(&self) -> Result<bool, ExecutionError> {
        let user = User::new();
        Ok(self.repo.insert(user).await?)
    }
}

/// TODO
#[derive(Debug, Display, Error, From, PartialEq, Eq)]
pub enum ExecutionError {
    /// TODO
    #[display("{_0}")]
    Repo(#[from] db::Error),
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
