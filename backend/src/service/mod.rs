//! TODO

mod create_user;
mod repo;

#[derive(Debug, Clone)]
pub(crate) struct Service<R> {
    repo: R,
}

impl<R> Service<R> {
    pub(crate) fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub(crate) trait UserService {
    type Error;

    fn create_user(
        &self,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}
