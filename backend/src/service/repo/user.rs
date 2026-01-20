//! User [`Repository`] implementations.

use crate::{
    domain::User,
    infra::{
        Repository,
        storage::{self, Storage},
    },
};

impl<DB: Sync> Repository<User> for Storage<DB> {
    type Error = storage::Error;

    async fn insert(&self, _user: User) -> Result<bool, Self::Error> {
        Ok(true)
    }
}
