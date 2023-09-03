use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

#[derive(Clone, GraphQLInputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, GraphQLObject)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, GraphQLObject, Serialize, Deserialize)]
pub struct GetUsers {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}
