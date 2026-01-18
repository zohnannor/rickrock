//! TODO

pub mod graphql;

/// TODO
pub trait UserService {
    /// TODO
    type Error;

    /// TODO
    fn create_user(
        &self,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}
