//! [`Repository`] of a [`Service`].
//!
//! [`Service`]: crate::service::Service

mod user;

#[cfg(test)]
pub mod mock {
    use std::sync::{Arc, nonpoison::Mutex};

    use crate::infra::{Repository, Transaction, storage};

    #[derive(Debug, Default, Clone)]
    pub struct Repo {
        users: Arc<Mutex<Vec<()>>>,
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
