//! [`Repository`] of a [`Service`].
//!
//! [`Service`]: crate::service::Service

use crate::{domain::User, infra::Repository, storage};

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
    pub struct Repo {
        users: Arc<Mutex<Vec<()>>>,
    }

    // Implemented manually to avoid implicit `T: Default` bound.
    impl Clone for Repo {
        fn clone(&self) -> Self {
            Self { users: Arc::clone(&self.users) }
        }
    }

    // Implemented manually to avoid implicit `T: Default` bound.
    impl Default for Repo {
        fn default() -> Self {
            Self { users: Arc::default() }
        }
    }

    impl<T> Repository<T> for Repo
    where
        T: Send,
    {
        type Error = storage::Error;

        async fn insert(&self, _entity: T) -> Result<bool, Self::Error> {
            self.users.lock().push(());
            Ok(true)
        }
    }

    impl Transaction for Repo {
        type Transacted = Self;
        type Error = storage::Error;

        async fn tx(&self) -> Result<Self::Transacted, Self::Error> {
            Ok(self.clone())
        }
    }
}
