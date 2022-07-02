use std::env;

use sea_orm::DatabaseConnection;

pub async fn create_db_connection_pool() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // establish connection to database and apply migrations
    sea_orm::Database::connect(&database_url).await.unwrap()
}
