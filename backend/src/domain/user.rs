//! Domain model of a [`User`] of a platform.

use derive_more::{From, Into};
use uuid::Uuid;

/// Domain model of a [`User`] of a platform.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct User {
    /// Unique ID of this [`User`].
    pub id: Id,
}

/// Unique identifier of a [`User`].
#[derive(Debug, Clone, Copy, From, Into, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(Uuid);

impl Id {
    /// Creates a new unique [`Id`].
    pub(crate) fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_ne;

    use super::*;

    impl User {
        /// Creates a new unique [`User`] for testing purposes.
        fn new() -> Self {
            Self { id: Id::new() }
        }
    }

    #[test]
    fn new_user_has_unique_id() {
        let user = User::new();
        let user2 = User::new();

        assert_ne!(user.id, user2.id);
    }
}

mod graphql {
    //! Conversions between [`domain`](crate::domain) and [`graphql`] types.

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
