use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::app_state::AppState;
use crate::utils::http_authorization_token::HttpAuthorizationToken;
use crate::utils::response_error::ResponseError;
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

pub async fn get_users(
    data: web::Data<AppState>,
) -> Result<HttpResponse, ResponseError> {
    let connection = &mut *data.connection.lock().unwrap();
    let mut users_service = UsersService::new(connection);

    let users = users_service
        .get_all_users()
        .map_err(|_| ResponseError::InternalError)?;
    
    let mut users_response = Vec::new();
    for user in users {
        users_response.push(UserResponse::from_user(&user));
    }

    Ok(HttpResponse::Ok().json(users_response))
}

pub async fn get_user_by_id(
    data: web::Data<AppState>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, ResponseError> {
    let connection = &mut *data.connection.lock().unwrap();
    let mut users_service = UsersService::new(connection);

    let user = users_service
        .get_user_by_id(*user_id)
        .map_err(|_| ResponseError::ErrorNotFound { entity: "User".to_string() })?;
    
    Ok(HttpResponse::Ok().json(UserResponse::from_user(&user)))
}

pub async fn get_me(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ResponseError> {
    let connection = &mut *data.connection.lock().unwrap();
    let mut users_service = UsersService::new(connection);

    let token = HttpAuthorizationToken::get_token(&req)
        .map_err(|error| {
            eprintln!("Error acquiring token: {:?}", error);
            ResponseError::ErrorUnauthorized { message: "Invalid token".to_string() }
        })?;
    let user_jwt = UserJWT::new(&data.jwt);
    let claims = user_jwt.get_claims(&token)
        .map_err(|error| {
            eprintln!("Error decoding token: {:?}", error);
            ResponseError::ErrorUnauthorized { message: "Invalid token".to_string() }
        })?;

    let user = users_service
        .get_user_by_id(claims.user_id)
        .map_err(|_| ResponseError::ErrorNotFound { entity: "User".to_string() })?;
    
    Ok(HttpResponse::Ok().json(UserResponse::from_user(&user)))
}

#[derive(Serialize, Deserialize)]
pub struct NewUserPayload {
    username: String,
    password: String,
}

pub async fn create_user(
    data: web::Data<AppState>,
    json_data: web::Json<NewUserPayload>,
) -> Result<HttpResponse, ResponseError> {
    let connection = &mut *data.connection.lock().unwrap();
    let mut users_service = UsersService::new(connection);

    let user = users_service
        .create_user(&NewUser {
            username: json_data.username.to_owned(),
            password: json_data.password.to_owned(),
        })
        .map_err(|_| ResponseError::ErrorNotFound { entity: "User".to_string() })?;
    
    Ok(HttpResponse::Ok().json(UserResponse::from_user(&user)))
}
