//! TODO

use derive_more::{From, Into};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct User {
    /// TODO
    pub(crate) id: Id,
}

#[derive(Debug, Clone, Copy, From, Into)]
pub(crate) struct Id(Uuid);

impl User {
    /// TODO
    pub(crate) fn new() -> Self {
        Self { id: Id(Uuid::now_v7()) }
    }
}

mod graphql {
    //! TODO

    use super::{Id, User};
    use crate::graphql;

    impl From<Id> for graphql::Id {
        #[inline]
        fn from(id: Id) -> Self {
            Self(id.0)
        }
    }

    impl From<User> for graphql::User {
        #[inline]
        fn from(user: User) -> Self {
            Self { id: user.id.into() }
        }
    }
}
