use checkito::{FullGenerate, Generate, boxed::Boxed};

use crate::model::{
    foreign::checkito::{utc_date_time_generator, uuid_generator},
    user::{User, UserCreatedAt, UserId},
};

pub fn user_created_at() -> Boxed<UserCreatedAt> {
    utc_date_time_generator()
        .map(|utc_date_time| UserCreatedAt(utc_date_time))
        .boxed()
}

impl FullGenerate for UserId {
    type Item = UserId;

    type Generator = Boxed<UserId>;

    fn generator() -> Self::Generator {
        uuid_generator().map(|uuid| UserId(uuid)).boxed()
    }
}

impl FullGenerate for User {
    type Item = User;

    type Generator = Boxed<User>;

    fn generator() -> Self::Generator {
        let user_id = UserId::generator();
        let user_created_at = user_created_at();

        Generate::map((user_id, user_created_at), |(user_id, user_created_at)| {
            Self::new(user_id, user_created_at)
        })
        .boxed()
    }
}
