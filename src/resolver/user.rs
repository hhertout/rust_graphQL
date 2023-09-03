use crate::{
    entity::user::Model,
    schemas::user::{CreateUserInput, UserCreated},
    services::user::UserService,
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
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Database>::new())
}
