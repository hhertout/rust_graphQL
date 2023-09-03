use std::collections::HashMap;

use juniper::GraphQLObject;


#[derive(Clone, GraphQLObject)]
pub struct User {
    id: i32,
    name: String,
}

#[derive(Clone, Default)]
pub struct Database {
    users: HashMap<i32, User>,
}
impl Database {
    pub fn new() -> Database {
        let mut users = HashMap::new();
        users.insert(
            1,
            User {
                id: 1,
                name: "Aron".into(),
            },
        );
        users.insert(
            2,
            User {
                id: 2,
                name: "Bea".into(),
            },
        );
        Database { users }
    }
    pub fn get_user(&self, id: &i32) -> Option<&User> {
        self.users.get(id)
    }
}

impl juniper::Context for Database {}