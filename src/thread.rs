use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use entity::thread::{self, ActiveModel};
use entity::thread::Entity as Thread;
use crate::utils::{get_assistant_id, get_openai_token, connect_db};
use crate::openai::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct CompactMessage {
    pub role: String,
    pub message: String,
    pub create_at: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetThreadResponse {
    pub thread_id: String,
    pub title: String,
    pub assistant_id: String,
    pub status: String,
    pub messages: Vec<CompactMessage>,
}

fn get_messages(t: &thread::Model) -> Vec<CompactMessage> {
    let serialized_messages = t.messages.clone();

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
    messeges
}

#[get("/threads/{id}")]
pub async fn get_thread(id: web::Path<String>) -> impl Responder {
    let openai_token = get_openai_token().await.unwrap();
    
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
                let messeges = get_messages(&thread);

                let res = GetThreadResponse {
                    thread_id: thread.id.clone(),
                    title: thread.title.clone(),
                    assistant_id: thread.assistant_id.clone(),
                    status: thread.status.clone(),
                    messages: messeges,
                };

                HttpResponse::Ok().json(res)   
            } else {
                // retrieve run status from OpenAI
                let client = awc::Client::default();
                
                let response = client.get(format!("https://api.openai.com/v1/threads/{}/runs/{}", &thread.id, &thread.run_id))
                    .insert_header(("Content-Type", "application/json"))
                    .insert_header(("Authorization", format!("Bearer {}", &openai_token)))
                    .insert_header(("OpenAI-Beta", "assistants=v1"))
                    .send()
                    .await;

                match response {
                    Ok(mut res) => {
                        #[derive(Deserialize, Debug)]
                        struct ResponseBody {
                            // pub id: String,
                            // pub object: String,
                            // pub created_at: i64,
                            // pub assistant_id: String,
                            // pub thread_id: String,
                            pub status: String,
                        }

                        let run_body = res.json::<ResponseBody>().await.unwrap();
                        
                        if run_body.status == "completed" {
                            // get thread messages from OpenAI
                            let response = client.get(format!("https://api.openai.com/v1/threads/{}/messages", &thread.id))
                                .insert_header(("Content-Type", "application/json"))
                                .insert_header(("Authorization", format!("Bearer {}", &openai_token)))
                                .insert_header(("OpenAI-Beta", "assistants=v1"))
                                .send()
                                .await;
                                  
                            #[derive(Deserialize, Debug)]
                            struct ResponseBody {
                                // pub object: String,
                                pub data: Vec<Message>,
                                // pub first_id: String,
                                // pub last_id: String,
                                // pub has_more: bool,
                            }

                            match response {
                                Ok(mut res) => {
                                    let body = res.json::<ResponseBody>().await.unwrap();
                                    println!("Get messages: {:?}", &body);

                                    // update thread status to complete
                                    let mut thread: thread::ActiveModel = thread.into();
                                    thread.status = Set("done".to_string());

                                    let messages = body.data;
                                    let mut compact_messages: Vec<CompactMessage> = vec![];
                                    for message in messages {
                                        compact_messages.push(CompactMessage {
                                            role: message.role.clone(),
                                            message: message.content[0].text.value.clone(),
                                            create_at: message.created_at,
                                        });
                                    }

                                    let serialized_messages = serde_json::to_string(&compact_messages).unwrap();
                                    thread.messages = Set(serialized_messages);

                                    let result = thread.save(&db).await;
                                    match result {
                                        Ok(thread) => {
                                            println!("Thread status updated to complete: {:?}", &id);
                                            let res = GetThreadResponse {
                                                thread_id: thread.get(thread::Column::Id).unwrap().to_string(),
                                                title: thread.get(thread::Column::Title).unwrap().to_string(),
                                                assistant_id: thread.get(thread::Column::AssistantId).unwrap().to_string(),
                                                status: "done".to_string(),
                                                messages: compact_messages,
                                            };

                                            HttpResponse::Ok().json(res)
                                        },
                                        Err(err) => {
                                            println!("Error updating thread status {:?}", err);
                                            HttpResponse::InternalServerError().body("Error updating thread status")
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("Error calling OpenAI: {:?}", e);
                                    HttpResponse::InternalServerError().body("Error calling OpenAI")
                                }
                            }
                        }else{
                            // return thread messages from database
                            let messeges = get_messages(&thread);

                            let res = GetThreadResponse {
                                thread_id: thread.id.clone(),
                                title: thread.title.clone(),
                                assistant_id: thread.assistant_id.clone(),
                                status: thread.status.clone(),
                                messages: messeges,
                            };

                            HttpResponse::Ok().json(res)
                        }
                    }
                    Err(e) => {
                        println!("Error calling OpenAI: {:?}", e);
                        HttpResponse::InternalServerError().body("Error calling OpenAI")
                    }
                }
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
    run_id: String,
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
    struct RequestBody {}
    let req = RequestBody {};      

    #[derive(Deserialize, Debug)]
    struct ResponseBody {
        pub id: String,
        // pub object: String,
        // pub created_at: i64,
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
                run_id: Set("".to_string()),
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
                        run_id: thread.run_id.clone(),
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
        // pub id: String,
        // pub object: String,
        // pub deleted: bool,
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
    let assistant_id = get_assistant_id().await.unwrap();

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

            // create new OpenAI run
            #[derive(Serialize)]
            struct RunRequestBody {
                pub assistant_id: String,
            }
            let run_req = RunRequestBody {
                assistant_id: assistant_id.clone(),
            };
            let response = client.post(format!("https://api.openai.com/v1/threads/{}/runs", &req_body.thread_id))
                .insert_header(("Content-Type", "application/json"))
                .insert_header(("Authorization", format!("Bearer {}", &openai_token)))
                .insert_header(("OpenAI-Beta", "assistants=v1"))
                .send_json(&run_req)
                .await;

            match response {
                Ok(mut res) => {
                    #[derive(Deserialize, Debug)]
                    struct RunResponseBody {
                        pub id: String,
                        // pub object: String,
                        // pub created_at: i64,
                        // pub assistant_id: String,
                        // pub thread_id: String,
                        // pub status: String,
                        // pub model: String,
                        // pub instructions: Option<String>,
                    }
                    let run_body = res.json::<RunResponseBody>().await.unwrap();
                    println!("Created run on OpenAI: {:?}", &run_body);

                    // update thread run_id in database
                    let db = connect_db().await.unwrap();

                    let thread = Thread::find_by_id(&req_body.thread_id)
                        .one(&db)
                        .await
                        .unwrap();

                    match thread {
                        Some(thread)=>{
                            let old_messages = thread.messages.clone();

                            let mut thread: thread::ActiveModel = thread.into();
                            thread.status = Set("waiting".to_string());
                            thread.run_id = Set(run_body.id.clone());

                            // append message to thread messages
                            let mut messages: Vec<CompactMessage> = if old_messages.is_empty() {
                                vec![]
                            } else {
                                match serde_json::from_str(&old_messages) {
                                    Ok(messages) => messages,
                                    Err(err) => {
                                        println!("Error deserializing messages: {:?}", err);
                                        vec![]
                                    }
                                }
                            };

                            messages.push(CompactMessage {
                                role: body.role.clone(),
                                message: body.content[0].text.value.clone(),
                                create_at: body.created_at,
                            });

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
                }
                Err(e) => {
                    println!("Error calling OpenAI: {:?}", e);
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