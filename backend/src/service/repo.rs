//! [`Repository`] of a [`Service`].
//!
//! [`Service`]: crate::service::Service

use crate::{domain::user::User, infra::Repository, storage};

impl<DB: Sync> Repository<User> for storage::Storage<DB> {
    type Error = storage::Error;

    async fn insert(&self, _user: User) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

#[cfg(test)]
pub mod mock {
    use std::sync::{Arc, nonpoison::Mutex};

    use super::*;
    use crate::infra::Transaction;

    #[derive(Debug)]
    pub struct Repo<T> {
        users: Arc<Mutex<Vec<T>>>,
    }

    // Implemented manually to avoid implicit `T: Default` bound.
    impl<T> Clone for Repo<T> {
        fn clone(&self) -> Self {
            Self { users: Arc::clone(&self.users) }
        }
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
        type Error = storage::Error;

        async fn insert(&self, entity: T) -> Result<bool, Self::Error> {
            self.users.lock().push(entity);
            Ok(true)
        }
    }

    impl<T> Transaction for Repo<T>
    where
        T: Send,
    {
        type Transacted = Self;
        type Error = storage::Error;

        async fn tx(&self) -> Result<Self::Transacted, Self::Error> {
            Ok(self.clone())
        }
    }
}
