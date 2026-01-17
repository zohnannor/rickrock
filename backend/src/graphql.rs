//! TODO

use juniper::{EmptySubscription, RootNode, graphql_object};

use crate::db;

/// TODO
#[derive(Debug)]
pub(crate) struct Query {
    pub db: db::Db,
}

#[graphql_object]
impl Query {
    /// TODO
    pub(crate) fn hello_world() -> String {
        "Hello, World!".to_owned()
    }
}

/// TODO
#[derive(Debug)]

pub(crate) struct Mutation {
    pub db: db::Db,
}

#[graphql_object]
impl Mutation {
    /// TODO
    pub(crate) fn hello_world() -> String {
        "Hello, World!".to_owned()
    }
}

/// TODO
pub(crate) type Schema = RootNode<Query, Mutation, EmptySubscription>;
