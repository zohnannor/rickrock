//! GraphQL API services.

pub mod graphql;

#[cfg(doc)]
use crate::api::graphql::User;

/// GraphQL API service for [`User`]s.
pub trait UserService {
    /// Error produced by this [`UserService`].
    type Error;

    /// Creates a new [`User`] of a platform.
    fn create_user(
        &self,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}
