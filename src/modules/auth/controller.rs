use actix_web::{post, Responder, web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::app_state::AppState;
use super::service::{AuthService, AuthServiceTrait};
use super::model::Credentials;
use super::super::users::service::{UserResponse, user_to_user_response};

#[derive(Serialize, Deserialize)]
struct CredentialsPayload {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct CredentialsResponse {
    jwt: String,
    user: UserResponse,
}

#[post("/login")]
async fn login(data: web::Data<AppState>, json_data: web::Json<CredentialsPayload>) -> impl Responder {
    let connection = &mut *data.connection.lock().unwrap();
    let mut auth_service = AuthService::new(connection);

    let result = auth_service
        .login(&Credentials {
            username: json_data.username.to_owned(),
            password: json_data.password.to_owned(),
        })
        .expect("Error loading users");
    
    HttpResponse::Ok().json(CredentialsResponse {
        jwt: result.jwt,
        user: user_to_user_response(&result.user),
    })
}
