use actix_web::{error, get, post, web, HttpRequest, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::app_state::AppState;
use crate::utils::req_authorization::ReqAuthorization;
use super::service::{UsersService, UsersServiceTrait};
use super::model::{NewUser, User};
use super::user_jwt::UserJWT;

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    username: String,
}

impl UserResponse {
    pub fn from_user(user: &User) -> UserResponse {
        UserResponse {
            username: user.username.to_owned(),
        }
    }
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
        let obj = UserResponse::from_user(&user);
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
    
    Ok(HttpResponse::Ok().json(UserResponse::from_user(&user)))
}

#[get("/me")]
async fn get_me(data: web::Data<AppState>, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let connection = &mut *data.connection.lock().unwrap();
    let mut users_service = UsersService::new(connection);

    let token = ReqAuthorization::get_token(&req)?;

    let user_jwt = UserJWT::new(&data.jwt);
    let claims = user_jwt.get_claims(&token)
        .map_err(|error| {
            eprintln!("Error decoding token: {:?}", error);
            actix_web::error::ErrorUnauthorized("Invalid token")
        })?;

    let user = users_service
        .get_user_by_id(claims.user_id)
        .map_err(|error| {
            eprintln!("{}", error);
            error::ErrorNotFound("User not found")
        })?;
    
    Ok(HttpResponse::Ok().json(UserResponse::from_user(&user)))
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
    
    Ok(HttpResponse::Ok().json(UserResponse::from_user(&user)))
}
