use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use entity::user::{self, ActiveModel};
use entity::user::Entity as User;

use crate::utils::connect_db;

#[derive(Deserialize, Debug)]
struct NewUserPayload {
    pub name: String,
    pub phone_number: String
}

#[derive(Deserialize, Debug)]
struct LoginPayload {
    pub phone_number: String
}

#[derive(Serialize, Debug)]
struct LoginResponse {
    pub customer_id: i32,
    pub name: String,
    pub phone_number: String,
}

#[post("/login")]
pub async fn login(req_body: web::Json<LoginPayload>) -> impl Responder {
    println!("Login user: {:?}", req_body);

    let db: DatabaseConnection = connect_db().await.unwrap();

    let user_info = User::find()
        .filter(user::Column::PhoneNumber.eq(&req_body.phone_number))
        .one(&db)
        .await;

    match user_info {
        Ok(info)=>{
            match info {
                Some(u)=>{
                    println!("User logged in {:?}", &u);
                    let res = LoginResponse {
                        customer_id: u.customer_id,
                        name: u.name,
                        phone_number: u.phone_number,
                    };
                    HttpResponse::Ok().json(res)
                },
                None=>{
                    println!("User not found {:?}", &req_body);
                    HttpResponse::NotFound().body("User not found")
                }
            }
        },
        Err(_)=>{
            println!("Error logging in user {:?}", &req_body);
            HttpResponse::InternalServerError().body("Error logging in user")
        }
    }
}

#[get("/users/{id}")]
pub async fn get_user(id: String) -> impl Responder {
    HttpResponse::Ok().body(format!("User ID: {}", id))
}

#[post("/users")]
pub async fn create_user(req_body: web::Json<NewUserPayload>) -> impl Responder {
    println!("Create user: {:?}", req_body);
    let db = connect_db().await.unwrap();
    
    let new_user = ActiveModel {
        name: Set(req_body.name.clone()),
        phone_number: Set(req_body.phone_number.clone()),
        ..Default::default()
    };

    println!("New user: {:?}", new_user);

    let result = new_user.insert(&db).await;

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
