use crate::{entity::user::Model, services::user::UserService};

use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use crate::config::db::Database;

// Queries represent the callable funcitons
pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }
    async fn get_users(context: &Database) -> Option<Vec<Model>> {
        UserService::new().find(&context.db_pool).await
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