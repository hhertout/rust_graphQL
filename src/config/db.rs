use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn db_connect() -> DatabaseConnection {
    let db_url = match env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => panic!("DB_URL is undefined"),
    };
    match Database::connect(db_url).await {
        Ok(db_pool) => db_pool,
        Err(_) => panic!("Failed to connect to database, please check your informations"),
    }
}