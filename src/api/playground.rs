use actix_web::{Error, HttpResponse};
use juniper_actix::{graphiql_handler, playground_handler};

pub async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphql", None).await
}
pub async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}
