use derive_more::{Display, Error, From};

use crate::{
    db::{self, Repository},
    service::{Service, UserService},
    user::User,
};

impl<R> UserService for Service<R>
where
    R: Repository<Error = db::Error> + Sync,
{
    type Error = Error;

    #[inline]
    async fn create_user(&self) -> Result<bool, Error> {
        let user = User::new();
        Ok(self.repo.insert_user(user).await?)
    }
}

#[derive(Debug, Display, Error, From, PartialEq, Eq)]
pub(crate) enum Error {
    Repo(#[from] db::Error),
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::service::repo::mock::MockRepo;

    #[tokio::test]
    async fn creates_user() {
        let service = Service::new(MockRepo::default());

        let res = service.create_user().await;

        assert_eq!(res, Ok(true), "user should be created");
    }
}
