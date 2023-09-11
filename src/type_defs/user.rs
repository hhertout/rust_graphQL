use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Clone, GraphQLInputObject)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, GraphQLInputObject)]
pub struct UpdateUserInput {
    pub email: String,
    pub name: String,
}

#[derive(Clone, GraphQLObject)]
pub struct UserCreated {
    pub name: String,
    pub email: String,
}

#[derive(Clone, GraphQLObject)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}


