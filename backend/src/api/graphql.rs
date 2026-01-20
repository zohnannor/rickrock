//! GraphQL API schemas and types.

use derive_more::{From, Into};
use juniper::{
    FieldError, GraphQLObject, GraphQLScalar, IntoFieldError, Value,
    graphql_object,
};
use uuid::Uuid;

use crate::{
    domain,
    service::command::{CreateUser, Executor},
};

/// Unique identifier of a [`User`].
#[derive(Debug, Clone, Copy, From, Into, GraphQLScalar)]
#[graphql(transparent)]
pub struct Id(pub Uuid);

#[expect(dead_code, reason = "WIP")]
/// A [`User`] of a platform.
#[derive(GraphQLObject)]
pub struct User {
    /// Unique ID of this [`User`].
    pub id: Id,
}

/// Root of GraphQL API queries.
#[derive(Debug)]
pub struct Query<S> {
    /// Service of GraphQL API queries.
    pub _service: S,
}

#[graphql_object]
impl<S> Query<S> {
    /// Returns a constant string.
    fn hello_world() -> String {
        "Hello, World!".to_owned()
    }
}

/// Root of GraphQL API mutations.
#[derive(Debug)]

pub struct Mutation<S> {
    /// Service of GraphQL API mutations.
    pub service: S,
}

#[expect(
    clippy::single_call_fn,
    reason = "GraphQL API resolvers are called by the server"
)]
#[graphql_object]
impl<S> Mutation<S>
where
    Self: Sync,
    S: Executor<domain::User, CreateUser, Output = bool>,
    Error: From<<S as Executor<domain::User, CreateUser>>::Error>,
{
    /// Creates a new [`User`] of a platform.
    async fn create_user(&self) -> Result<bool, Error> {
        Ok(self.service.run(CreateUser).await?)
    }
}

/// Error produced by GraphQL API resolvers.
pub struct Error {
    /// Error message.
    pub message: String,
}

impl<S> IntoFieldError<S> for Error {
    fn into_field_error(self) -> FieldError<S> {
        FieldError::new(self.message, Value::Null)
    }
}
