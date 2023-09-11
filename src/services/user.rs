use crate::entity::user;
use crate::entity::user::ActiveModel;
use crate::entity::user::Model;

use crate::entity::user::Entity as User;
use crate::type_defs::user::CreateUserInput;
use crate::type_defs::user::UpdateUserInput;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
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
    pub async fn find_by_email(&self, db_pool: &DatabaseConnection, email: &str) -> Option<Model> {
        let user = User::find()
            .filter(user::Column::Email.contains(email))
            .one(db_pool)
            .await;
        match user {
            Ok(user) => user,
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
    pub async fn create_user(
        &self,
        db_pool: &DatabaseConnection,
        user: CreateUserInput,
    ) -> Option<ActiveModel> {
        let user = user::ActiveModel {
            email: Set(user.email.to_owned()),
            password: Set(user.password.to_owned()),
            name: Set(user.name.to_owned()),
            ..Default::default()
        };
        user.save(db_pool).await.ok()
    }
    pub async fn update_user(
        &self,
        db_pool: &DatabaseConnection,
        user_updated: UpdateUserInput,
    ) -> Option<Model> {
        let user = &self.find_by_email(db_pool, &user_updated.email).await;
        if user.is_none() {
            None
        } else {
            let mut user: user::ActiveModel = user.clone().unwrap().into();
            user.name = Set(user_updated.name.to_owned());

            let user = user.update(db_pool).await.unwrap();
            Some(user)
        }
    }
}
