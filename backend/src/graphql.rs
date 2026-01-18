//! TODO

use derive_more::{Display, Error, From, Into};
use juniper::{
    FieldError, GraphQLObject, GraphQLScalar, IntoFieldError, graphql_object,
};
use uuid::Uuid;

use crate::{service::UserService, user};

#[derive(Debug, Clone, Copy, From, Into, GraphQLScalar)]
#[graphql(transparent)]
pub(crate) struct Id(pub(crate) Uuid);

#[derive(GraphQLObject)]
pub(crate) struct User {
    pub(crate) id: Id,
}

/// TODO
#[derive(Debug)]
pub(crate) struct Query<S> {
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

pub(crate) struct Mutation<S> {
    pub(crate) service: S,
}

#[graphql_object]
impl<S> Mutation<S>
where
    S: UserService<Error: ToString>,
{
    async fn create_user(&self) -> Result<bool, String> {
        Ok(self.service.create_user().await.map_err(|e| e.to_string())?)
    }
}
