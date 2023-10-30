use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::utils::response_error::ResponseError;
use crate::app_state::AppState;
use super::model::Credentials;
use super::super::users::controller::UserResponse;
use super::service::AuthServiceTrait;

#[derive(Serialize, Deserialize)]
pub struct CredentialsPayload {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct CredentialsResponse {
    jwt: String,
    user: UserResponse,
}

pub async fn login(
    data: web::Data<AppState>,
    json_data: web::Json<CredentialsPayload>,
) -> Result<HttpResponse, ResponseError> {
    let result = data.auth_service.lock().unwrap()
        .login(&Credentials {
            username: json_data.username.to_owned(),
            password: json_data.password.to_owned(),
        })
        .map_err(|_| ResponseError::ValidationError { field: "password".to_string() })?;
    
    Ok(HttpResponse::Ok().json(CredentialsResponse {
        jwt: result.jwt,
        user: UserResponse::from_user(&result.user),
    }))
}
