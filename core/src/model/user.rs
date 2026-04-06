use time::UtcDateTime;
use uuid::Uuid;

pub mod foreign;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub created_at: UserCreatedAt,
}

impl User {
    pub fn new(id: UserId, created_at: UserCreatedAt) -> Self {
        Self { id, created_at }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserCreatedAt(pub UtcDateTime);
