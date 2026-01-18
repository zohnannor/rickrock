//! TODO

use derive_more::{From, Into};
use uuid::Uuid;

/// TODO
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    /// TODO
    pub id: Id,
}

/// TODO
#[derive(Debug, Clone, Copy, From, Into, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(Uuid);

impl Id {
    /// TODO
    fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl User {
    /// TODO
    pub fn new() -> Self {
        Self { id: Id::new() }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_ne;

    use super::*;

    #[test]
    fn new_user_has_unique_id() {
        let user = User::new();
        let user2 = User::new();

        assert_ne!(user.id, user2.id);
    }
}

mod graphql {
    //! TODO

    use super::{Id, User};
    use crate::api::graphql;

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
