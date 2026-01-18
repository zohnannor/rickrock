use crate::{
    db::{self, Repository},
    user::User,
};

impl Repository for db::Db {
    type Error = db::Error;

    async fn insert_user(&self, _user: User) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

#[cfg(test)]
pub(crate) mod mock {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use uuid::Uuid;

    use crate::{
        db::{self, Repository},
        user::User,
    };

    #[derive(Debug, Default, Clone)]
    pub(crate) struct MockRepo {
        users: Arc<Mutex<HashMap<Uuid, User>>>,
    }

    impl Repository for MockRepo {
        type Error = db::Error;

        async fn insert_user(&self, user: User) -> Result<bool, Self::Error> {
            let _ = self.users.lock().unwrap().insert(user.id.into(), user);
            Ok(true)
        }
    }
}
