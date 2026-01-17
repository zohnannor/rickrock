//! TODO

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

impl Db {}
