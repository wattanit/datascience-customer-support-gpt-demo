mod user;
mod utils;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use user::{get_user, create_user, login};

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
            .service(login)
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