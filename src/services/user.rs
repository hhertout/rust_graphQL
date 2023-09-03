use sea_orm::DatabaseConnection;

use crate::schemas::user::CreateUserInput;

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_user(&self, db_connection: &mut DatabaseConnection, user: CreateUserInput) {}
}
