//! TODO

use crate::{
    db::{self, Repository},
    domain::user::User,
};

impl Repository<User> for db::Db {
    type Error = db::Error;

    async fn insert(&self, _user: User) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

#[cfg(test)]
pub mod mock {
    use std::sync::{Arc, nonpoison::Mutex};

    use super::*;

    #[derive(Debug, Clone)]
    pub struct Repo<T> {
        users: Arc<Mutex<Vec<T>>>,
    }

    // Implemented manually to avoid implicit `T: Default` bound.
    impl<T> Default for Repo<T> {
        fn default() -> Self {
            Self { users: Arc::default() }
        }
    }

    impl<T> Repository<T> for Repo<T>
    where
        T: Send,
    {
        type Error = db::Error;

        async fn insert(&self, entity: T) -> Result<bool, Self::Error> {
            self.users.lock().push(entity);
            Ok(true)
        }
    }
}
