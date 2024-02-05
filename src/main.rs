mod user;
mod thread;
mod utils;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use user::{get_user, create_user, login};
use thread::{get_thread, get_threads, create_thread, create_message};

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