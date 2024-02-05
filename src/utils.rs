use sea_orm::{Database, DatabaseConnection, DbErr};
use dotenv::dotenv;
use std::env;

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> { // Replace `sqlx::Error` with `Error`
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(database_url).await?;
    Ok(db)
}

pub async fn get_openai_token() -> Result<String, String> {
    dotenv().ok();
    let token = env::var("OPENAI_TOKEN").expect("OPENAI_TOKEN must be set");
    Ok(token)
}

pub async fn get_assistant_id() -> Result<String, String> {
    dotenv().ok();
    let assistant_id = env::var("ASSISTANT_TH_ID").expect("ASSISTANT_TH_ID must be set");
    Ok(assistant_id)
}