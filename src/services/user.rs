use crate::entity::user::Model;

use crate::entity::user::Entity as User;
use sea_orm::{DatabaseConnection, EntityTrait};

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn find(&self, db_pool: &DatabaseConnection) -> Option<Vec<Model>> {
        let users = User::find().all(db_pool).await;
        match users {
            Ok(users) => Some(users),
            Err(_) => None,
        }
    }
}
