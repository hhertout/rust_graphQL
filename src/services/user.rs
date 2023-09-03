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
    pub async fn find_by_id(&self, db_pool: &DatabaseConnection, id: i32) -> Option<Model> {
        let user = User::find_by_id(id).one(db_pool).await;
        match user {
            Ok(user) => user,
            Err(_) => None,
        }
    }
}
