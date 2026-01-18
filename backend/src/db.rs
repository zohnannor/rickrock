//! TODO

use derive_more::{Display, Error};

use crate::user::User;

/// TODO
#[derive(Debug, Clone)]
pub(crate) struct Db {
    /// TODO
    pool: sqlx::PgPool,
}

impl Db {
    /// TODO
    #[expect(clippy::single_call_fn, reason = "TODO")]
    pub(crate) const fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

pub(crate) trait Repository {
    type Error;

    fn insert_user(
        &self,
        user: User,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}

#[derive(Debug, Display, Error, PartialEq, Eq)]
pub(crate) enum Error {}
