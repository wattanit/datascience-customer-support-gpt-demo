use sea_orm::{Database, DatabaseConnection, DbErr};
use dotenv::dotenv;
use std::env;

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> { // Replace `sqlx::Error` with `Error`
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(database_url).await?;
    Ok(db)
}