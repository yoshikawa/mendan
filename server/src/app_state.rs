use sea_orm::DatabaseConnection;

use crate::db;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub async fn init() -> Self {
        Self {
            db: db::create_db_connection_pool().await,
        }
    }
}
