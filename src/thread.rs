use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use crate::utils::{get_assistant_id, get_openai_token};

#[get("/threads")]
pub async fn get_threads() -> impl Responder {
    HttpResponse::Ok().body("Get threads list")
}

#[get("/threads/{id}")]
pub async fn get_thread(id: String) -> impl Responder {
    // check if thread status is complete

    // if not, get thread messages from OpenAI

    // update thread status to complete

    // return response

    HttpResponse::Ok().body(format!("Get thread {}", id))
}

// create a new thread
#[post("/threads")]
pub async fn create_thread(req_body: String) -> impl Responder {
    // get assistant ID
    let openai_token = get_openai_token().await.unwrap();
    let assistant_id = get_assistant_id().await.unwrap();

    // create new thread on OpenAI
    let mut client = awc::Client::default();

    #[derive(Serialize)]
    struct RequestBody {};
    let req = RequestBody {};

    let response = client.post("https://api.openai.com/v1/threads")
        .insert_header(("OpenAI-Beta", "assistants=v1"))
        .send_json(&req)
        .await;
    // let request = serde_json::json!({
    //     "lang": "rust",
    //     "body": "json"
    // });
    


    // create new thread entry in database

    // return response


    HttpResponse::Ok().body("Create thread")
}

#[post("/threads/{id}/message")]
pub async fn create_message(id: String, req_body: String) -> impl Responder {
    // send message to OpenAI thread

    // update thread status to waiting

    // return response

    HttpResponse::Ok().body(format!("Create message in thread {}", id))
}