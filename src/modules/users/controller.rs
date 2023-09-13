use actix_web::{get, Responder, web, HttpResponse, error};
use crate::app_state::AppState;

use serde::Serialize;
use super::service::{UsersService, UsersServiceTrait};


#[derive(Serialize)]
struct UserResponse {
    username: String,
}

#[get("/")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let connection = &mut *data.connection.lock().unwrap();
    let mut users_service = UsersService::new(connection);

    let users = users_service
        .get_all_users()
        .expect("Error loading users");
    
    let mut users_response = Vec::new();
    for user in users {
        let obj = UserResponse {
            username: user.username,
        };
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
    
    let user_response: UserResponse = UserResponse {
        username: user.username,
    };

    Ok(HttpResponse::Ok().json(user_response))
}
