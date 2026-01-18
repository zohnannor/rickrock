//! TODO

mod create_user;
mod repo;

/// TODO
#[derive(Debug, Clone)]
pub struct Service<R> {
    /// TODO
    repo: R,
}

impl<R> Service<R> {
    /// TODO
    pub(crate) const fn new(repo: R) -> Self {
        Self { repo }
    }
}
