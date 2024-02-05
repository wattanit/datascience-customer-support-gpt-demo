use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Serialize, Deserialize};
use crate::utils::{get_assistant_id, get_openai_token, connect_db};
use entity::thread::ActiveModel;
use entity::thread::Entity as Thread;

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

#[derive(Deserialize, Debug)]
pub struct CreateThreadPayload {
    pub name: String,
    pub customer_id: i32,
}

// create a new thread
#[post("/threads")]
pub async fn create_thread(req_body: web::Json<CreateThreadPayload>) -> impl Responder {
    // get assistant ID
    let openai_token = get_openai_token().await.unwrap();
    let assistant_id = get_assistant_id().await.unwrap();

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
            println!("Created thread on OpenAI: {:?}", &body);

            // create new thread entry in database
            let db = connect_db().await.unwrap();
            
            let new_thread = ActiveModel {
                id: Set(body.id.clone()),
                title: Set(format!("สวัสดีค่ะ คุณ {}", &req_body.name)),
                assistant_id: Set(assistant_id),
                customer_id: Set(req_body.customer_id),
            };

            let result = new_thread.insert(&db).await;

            match result {
                Ok(thread) => {
                    println!("Thread created: {:?}", thread);
                    HttpResponse::Ok().body("Create thread")
                },
                Err(err) => {
                    println!("Error creating thread {:?}", err);
                    println!("However, OpenAI thread has been created id: {}", &body.id);
                    HttpResponse::InternalServerError().body("Error creating thread")
                }
            }
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
            let db = connect_db().await.unwrap();

            let result = Thread::delete_by_id(&id.clone())
                .exec(&db)
                .await;

            match result {
                Ok(_) => {
                    println!("Thread deleted: {:?}", &id);
                    HttpResponse::Ok().body(format!("Delete thread {}", &id))
                },
                Err(err) => {
                    println!("Error deleting thread {:?}", err);
                    println!("However, OpenAI thread has been deleted id: {}", &id);
                    HttpResponse::InternalServerError().body(format!("Error deleting thread {}", &id))
                }
            }
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