use crate::{entity::user::Model, services::user::UserService};

use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use crate::config::db::Database;


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

pub type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}