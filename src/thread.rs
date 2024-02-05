use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::utils::{get_assistant_id, get_openai_token, connect_db};

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
    // let assistant_id = get_assistant_id().await.unwrap();

    // create new thread on OpenAI
    let client = awc::Client::default();

    #[derive(Serialize)]
    struct RequestBody {};
    let req = RequestBody {};

    #[derive(Deserialize, Debug)]
    struct ResponseBody {
        pub id: String,
        pub object: String,
        pub created_at: i64,
    }

    let response = client.post("https://api.openai.com/v1/threads")
        .insert_header(("Content-Type", "application/json"))
        .insert_header(("Authorization", format!("Bearer {}", &openai_token)))
        .insert_header(("OpenAI-Beta", "assistants=v1"))
        .send_json(&req)
        .await;

    match response {
        Ok(mut res) => {
            let body = res.json::<ResponseBody>().await.unwrap();
            // let body = res.body().await.unwrap();
            println!("Created thread on OpenAI: {:?}", &body);

            // let db = connect_db().await.unwrap();

            // create new thread entry in database

            // return response
            HttpResponse::Ok().body("Create thread")
        }
        Err(e) => {
            println!("Error calling OpenAI: {:?}", e);
            HttpResponse::InternalServerError().body("Error calling OpenAI")
        }
    }
}

#[delete("/threads/{id}")]
pub async fn delete_thread(id: web::Path<String>) -> impl Responder {
    // delete thread from OpenAI
    println!("Delete thread on OpenAI: {}", &id);

    let openai_token = get_openai_token().await.unwrap();

    let client = awc::Client::default();

    // {
    //     "id": "thread_abc123",
    //     "object": "thread.deleted",
    //     "deleted": true
    //   }
    #[derive(Deserialize, Debug)]
    struct ResponseBody {
        pub id: String,
        pub object: String,
        pub deleted: bool,
    }

    let response = client.delete(format!("https://api.openai.com/v1/threads/{}", &id))
        .insert_header(("Content-Type", "application/json"))
        .insert_header(("Authorization", format!("Bearer {}", &openai_token)))
        .insert_header(("OpenAI-Beta", "assistants=v1"))
        .send().await;

    match response {
        Ok(mut res) => {
            let body = res.json::<ResponseBody>().await.unwrap();
            println!("Delete thread: {:?}", &body);

            // delete thread from database

            // return response
            HttpResponse::Ok().body(format!("Delete thread {}", &id))
        }
        Err(e) => {
            println!("Error calling OpenAI: {:?}", e);
            HttpResponse::InternalServerError().body("Error calling OpenAI")
        }
    }
}

#[post("/threads/{id}/message")]
pub async fn create_message(id: String, req_body: String) -> impl Responder {
    // send message to OpenAI thread

    // update thread status to waiting

    // return response

    HttpResponse::Ok().body(format!("Create message in thread {}", id))
}