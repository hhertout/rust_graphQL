#![deny(warnings)]

use actix_web::web::{self};
use actix_web::{Error, HttpResponse};
use juniper_actix::graphql_handler;

use crate::resolver::user::Schema;
use crate::config::db::Database;

pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Database::new().await;
    graphql_handler(&schema, &context, req, payload).await
}
