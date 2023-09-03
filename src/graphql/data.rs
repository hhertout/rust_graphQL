use sea_orm::DatabaseConnection;

use crate::config::db::db_connect;

#[derive(Clone)]
pub struct Database {
    pub db_pool: DatabaseConnection
}
impl Database {
    pub async fn new() -> Database {
        Database {
            db_pool: db_connect().await,
        }
    }
}

impl juniper::Context for Database {}