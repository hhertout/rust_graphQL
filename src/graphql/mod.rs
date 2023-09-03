use crate::entity::user::Model;

use super::entity::user::Entity as User;
use data::Database;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use sea_orm::EntityTrait;

pub mod data;

// Queries represent the callable funcitons
pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }
    async fn get_users(context: &Database) -> Option<Vec<Model>> {
        let users = User::find().all(&context.db_pool).await;
        match users {
            Ok(users) => Some(users),
            Err(_) => None,
        }
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}
