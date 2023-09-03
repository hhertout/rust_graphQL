use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

use self::data::{Database, User};

pub mod data;

// Queries represent the callable funcitons
pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    fn user(
        context: &Database,
        #[graphql(description = "id of the user")] id: i32,
    ) -> Option<&User> {
        context.get_user(&id)
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
