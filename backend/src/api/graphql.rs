//! TODO

use derive_more::{From, Into};
use juniper::{GraphQLObject, GraphQLScalar, graphql_object};
use uuid::Uuid;

use crate::api::UserService;

/// TODO
#[derive(Debug, Clone, Copy, From, Into, GraphQLScalar)]
#[graphql(transparent)]
pub struct Id(pub(crate) Uuid);

/// TODO
#[derive(GraphQLObject)]
pub struct User {
    /// TODO
    pub(crate) id: Id,
}

/// TODO
#[derive(Debug)]
pub struct Query<S> {
    /// TODO
    pub(crate) service: S,
}

#[graphql_object]
impl<S> Query<S> {
    /// TODO
    fn hello_world() -> String {
        "Hello, World!".to_owned()
    }
}

/// TODO
#[derive(Debug)]

pub struct Mutation<S> {
    /// TODO
    pub(crate) service: S,
}

#[graphql_object]
impl<S> Mutation<S>
where
    S: UserService<Error: ToString> + Sync,
{
    /// TODO
    async fn create_user(&self) -> Result<bool, String> {
        self.service.create_user().await.map_err(|e| e.to_string())
    }
}
