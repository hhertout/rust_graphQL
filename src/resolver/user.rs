use crate::{
    entity::user::Model,
    services::user::UserService,
    type_defs::user::{CreateUserInput, UpdateUserInput, UserCreated},
};

use crate::config::db::Database;
use juniper::{graphql_object, EmptySubscription, RootNode};
use sea_orm::TryIntoModel;

type User = Model;
// Queries represent the callable funcitons
pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    async fn get_users(context: &Database) -> Option<Vec<User>> {
        UserService::new().find(&context.db_pool).await
    }
    async fn get_user_by_id(context: &Database, id: i32) -> Option<User> {
        UserService::new().find_by_id(&context.db_pool, id).await
    }
    async fn get_user_by_email(context: &Database, email: String) -> Option<User> {
        UserService::new()
            .find_by_email(&context.db_pool, &email)
            .await
    }
}

pub struct Mutation;
#[graphql_object(context = Database)]
impl Mutation {
    async fn create_user(context: &Database, infos: CreateUserInput) -> Option<UserCreated> {
        let user = UserService::new()
            .create_user(&context.db_pool, infos)
            .await;
        match user {
            Some(user) => {
                let user_created = user.try_into_model().unwrap();
                Some(UserCreated {
                    name: user_created.name,
                    email: user_created.email,
                })
            }
            None => None,
        }
    }
    async fn update_user(context: &Database, infos: UpdateUserInput) -> Option<UserCreated> {
        let user = UserService::new()
            .update_user(&context.db_pool, infos)
            .await;
        match user {
            Some(user) => Some(UserCreated {
                name: user.name,
                email: user.email,
            }),
            None => None,
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Database>::new())
}
