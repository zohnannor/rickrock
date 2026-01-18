//! TODO

use derive_more::{Display, Error};

/// TODO
#[derive(Debug, Clone)]
pub struct Db {
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

/// TODO
pub trait Repository<T> {
    /// TODO
    type Error;

    /// TODO
    fn insert(
        &self,
        entity: T,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}

#[expect(
    clippy::error_impl_error,
    reason = "intended to be used as `db::Error`"
)]
/// TODO
#[derive(Debug, Display, Error, PartialEq, Eq)]
pub enum Error {}
