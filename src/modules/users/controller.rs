use actix_web::{get, post, Responder, web, HttpResponse, error};
use serde::{Serialize, Deserialize};
use crate::app_state::AppState;
use super::service::{UsersService, UsersServiceTrait, user_to_user_response};
use super::model::NewUser;

#[get("/")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let connection = &mut *data.connection.lock().unwrap();
    let mut users_service = UsersService::new(connection);

    let users = users_service
        .get_all_users()
        .expect("Error loading users");
    
    let mut users_response = Vec::new();
    for user in users {
        let obj = user_to_user_response(&user);
        users_response.push(obj);
    }

    HttpResponse::Ok().json(users_response)
}

#[get("/{user_id}")]
async fn get_user_by_id(data: web::Data<AppState>, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let connection = &mut *data.connection.lock().unwrap();
    let mut users_service = UsersService::new(connection);

    let user = users_service
        .get_user_by_id(*user_id)
        .map_err(|error| {
            eprintln!("{}", error);
            error::ErrorNotFound("User not found")
        })?;
    
    Ok(HttpResponse::Ok().json(user_to_user_response(&user)))
}


#[derive(Serialize, Deserialize)]
struct NewUserPayload {
    username: String,
    password: String,
}

#[post("/")]
async fn create_user(data: web::Data<AppState>, json_data: web::Json<NewUserPayload>) -> actix_web::Result<HttpResponse> {
    let connection = &mut *data.connection.lock().unwrap();
    let mut users_service = UsersService::new(connection);

    let user = users_service
        .create_user(&NewUser {
            username: json_data.username.to_owned(),
            password: json_data.password.to_owned(),
        })
        .map_err(|error| {
            eprintln!("{}", error);
            error::ErrorNotFound("Smth went wrong during user creation")
        })?;
    
    Ok(HttpResponse::Ok().json(user_to_user_response(&user)))
}
