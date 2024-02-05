use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use crate::utils::{get_assistant_id, get_openai_token, connect_db};
use entity::thread::{self, ActiveModel};
use entity::thread::Entity as Thread;

#[derive(Serialize, Deserialize, Debug)]
pub struct CompactMessage {
    pub role: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message{
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub thread_id: String,
    pub role: String,
    pub content: Vec<MessageContent>,
    pub assistant_id: Option<String>,
    pub run_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContent{
    // renamed field type to typeString
    #[serde(rename = "type")]
    pub type_strig: String,
    pub text: MessageContentText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageContentText{
    pub value: String,
    pub annotations: Vec<String>,
}

#[get("/threads")]
pub async fn get_threads() -> impl Responder {
    HttpResponse::Ok().body("Get threads list")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetThreadResponse {
    pub thread_id: String,
    pub title: String,
    pub assistant_id: String,
    pub status: String,
    pub messages: Vec<CompactMessage>,
}

#[get("/threads/{id}")]
pub async fn get_thread(id: web::Path<String>) -> impl Responder {
    
    // check if thread status is complete
    println!("Get thread: {}", &id);

    let db = connect_db().await.unwrap();

    let thread = Thread::find_by_id(&id.clone())
        .one(&db)
        .await
        .unwrap();

    match thread {
        Some(thread)=>{
            // let thread: thread::ActiveModel = thread.into();
            if thread.status == "done" {
                // return thread messages from database
                let serialized_messages = thread.messages;
                let messeges: Vec<CompactMessage> = if serialized_messages.is_empty() {
                    vec![]
                } else {
                    match serde_json::from_str(&serialized_messages) {
                        Ok(messages) => messages,
                        Err(err) => {
                            println!("Error deserializing messages: {:?}", err);
                            vec![]
                        }
                    }
                };

                let res = GetThreadResponse {
                    thread_id: thread.id.clone(),
                    title: thread.title.clone(),
                    assistant_id: thread.assistant_id.clone(),
                    status: thread.status.clone(),
                    messages: messeges,
                };

                HttpResponse::Ok().json(res)   
            } else {
                // get thread messages from OpenAI

                // update thread status to complete

                HttpResponse::Ok().body(format!("Get thread {}: RUNING", id))
            }
        },
        None => {
            HttpResponse::NotFound().body(format!("Thread not found {}", id))
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateThreadPayload {
    pub name: String,
    pub customer_id: i32,
}

#[derive(Serialize, Debug)]
pub struct CreateThreadResponse {
    thread_id: String,
    title: String,
    assistant_id: String,
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
                status: Set("done".to_string()),
                messages: Set("".to_string()),
            };

            let result = new_thread.insert(&db).await;

            match result {
                Ok(thread) => {
                    println!("Thread created: {:?}", thread);

                    let res = CreateThreadResponse {
                        thread_id: thread.id.clone(),
                        title: thread.title.clone(),
                        assistant_id: thread.assistant_id.clone(),
                    };

                    HttpResponse::Ok().json(res)
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

#[derive(Deserialize, Debug)]
pub struct CreateMessagePayload {
    pub thread_id: String,
    pub message: String,
}

#[post("/threads/messages")]
pub async fn create_message(req_body: web::Json<CreateMessagePayload>) -> impl Responder {
    // send message to OpenAI thread
    let openai_token = get_openai_token().await.unwrap();

    let client = awc::Client::default();

    #[derive(Serialize)]
    struct RequestBody {
        pub role: String,
        pub content: String,
    }

    let req = RequestBody {
        role: "user".to_string(),
        content: req_body.message.clone(),
    };

    let response = client.post(format!("https://api.openai.com/v1/threads/{}/messages", &req_body.thread_id))
        .insert_header(("Content-Type", "application/json"))
        .insert_header(("Authorization", format!("Bearer {}", &openai_token)))
        .insert_header(("OpenAI-Beta", "assistants=v1"))
        .send_json(&req)
        .await;

    match response {
        Ok(mut res) => {
            let body = res.json::<Message>().await.unwrap();
            println!("Created message on OpenAI: {:?}", &body);

            // update thread status to waiting
            let db = connect_db().await.unwrap();

            let thread = Thread::find_by_id(&req_body.thread_id)
                .one(&db)
                .await
                .unwrap();

            match thread {
                Some(thread)=>{
                    let mut thread: thread::ActiveModel = thread.into();
                    thread.status = Set("waiting".to_string());
                    
                    let messages = vec![CompactMessage {
                        role: body.role.clone(),
                        message: body.content[0].text.value.clone(),
                    }];

                    let serialized_messages = serde_json::to_string(&messages).unwrap();
                    thread.messages = Set(serialized_messages);

                    let result = thread.save(&db).await;
                    match result {
                        Ok(_) => {
                            println!("Thread status updated to waiting: {:?}", &req_body.thread_id);
                        },
                        Err(err) => {
                            println!("Error updating thread status {:?}", err);
                        }
                    }
                },
                None => {
                    println!("Thread not found: {:?}", &req_body.thread_id);
                }
            }

            // return response
            HttpResponse::Ok().body(format!("Create message in thread {}", &req_body.thread_id))
        }
        Err(e) => {
            println!("Error calling OpenAI: {:?}", e);
            HttpResponse::InternalServerError().body("Error calling OpenAI")
        }
    }
}