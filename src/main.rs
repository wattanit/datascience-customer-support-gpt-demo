use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, DbErr, Set};
use serde::Deserialize;
use dotenv::dotenv;
use std::env;
use entity::user::ActiveModel;

async fn connect_db() -> Result<DatabaseConnection, DbErr> { // Replace `sqlx::Error` with `Error`
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(database_url).await?;
    Ok(db)
}

#[get("/users/{id}")]
async fn get_user(id: String) -> impl Responder {
    HttpResponse::Ok().body(format!("User ID: {}", id))
}

#[derive(Deserialize, Debug)]
struct NewUser {
    pub name: String,
    pub phone_number: String
}

#[post("/users")]
async fn create_user(req_body: web::Json<NewUser>) -> impl Responder {
    println!("Create user: {:?}", req_body);
    let db = connect_db().await.unwrap();
    
    let new_user = ActiveModel {
        name: Set(req_body.name.clone()),
        phone_number: Set(req_body.phone_number.clone()),
        ..Default::default()
    };

    println!("New user: {:?}", new_user);

    let result = new_user.insert(&db).await;

    // let result = new_user.save(&db).await;

    match result {
        Ok(user) => {
            println!("User created: {:?}", user);
            HttpResponse::Ok().body("User created")
        },
        Err(err) => {
            println!("Error creating user {:?}", err);
            HttpResponse::InternalServerError().body("Error creating user")
        }
    }
    // HttpResponse::Ok().body("User created")
}

#[get("/threads")]
async fn get_threads() -> impl Responder {
    HttpResponse::Ok().body("Get threads list")
}

#[get("/threads/{id}")]
async fn get_thread(id: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Get thread {}", id))
}

// create a new thread
#[post("/threads")]
async fn create_thread(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Create thread")
}

#[post("/threads/{id}/message")]
async fn create_message(id: String, req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Create message in thread {}", id))
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("MyMo Customer Service Demo API")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Starting server at http://127.0.0.1:8080/");
    HttpServer::new(|| {
        let scope = web::scope("/api/v1")
            .service(get_user)
            .service(create_user)
            .service(get_threads)
            .service(get_thread)
            .service(create_thread)
            .service(create_message);
        App::new().service(scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}