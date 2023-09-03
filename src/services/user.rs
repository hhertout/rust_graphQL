use crate::entity::user;
use crate::entity::user::ActiveModel;
use crate::entity::user::Model;

use crate::entity::user::Entity as User;
use crate::type_defs::user::CreateUserInput;
use sea_orm::ActiveModelTrait;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::Set;

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
    pub async fn create_user(&self, db_pool: &DatabaseConnection, user: CreateUserInput) -> Option<ActiveModel> {
        let user = user::ActiveModel {
            email: Set(user.email.to_owned()),
            password: Set(user.password.to_owned()),
            name: Set(user.name.to_owned()),
            ..Default::default()
        };
        user.save(db_pool).await.ok()
    }
}
