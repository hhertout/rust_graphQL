use std::env;

use sea_orm::{Database as DB, DatabaseConnection};

#[derive(Clone)]
pub struct Database {
    pub db_pool: DatabaseConnection,
}
impl Database {
    pub async fn new() -> Database {
        Database {
            db_pool: db_connect().await,
        }
    }
}

impl juniper::Context for Database {}

pub async fn db_connect() -> DatabaseConnection {
    let db_url = match env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => panic!("DB_URL is undefined"),
    };
    match DB::connect(db_url).await {
        Ok(db_pool) => db_pool,
        Err(_) => panic!("Failed to connect to database, please check your informations"),
    }
}
