use actix_web::{web, Scope};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::utils::auth_validator::auth_validator;

pub mod controller;
pub mod model;
pub mod service;
pub mod user_jwt;

pub fn get_users_routes() -> Scope {
    let auth_validator_middleware = HttpAuthentication::bearer(auth_validator);

    web::scope("/users")
        .route("", web::get().to(controller::get_users)
            .wrap(auth_validator_middleware.clone()))
        .route("/me", web::get().to(controller::get_me)
            .wrap(auth_validator_middleware.clone()))
        .route("/{user_id}", web::get().to(controller::get_user_by_id)
            .wrap(auth_validator_middleware.clone()))
        .route("", web::post().to(controller::create_user))
}
