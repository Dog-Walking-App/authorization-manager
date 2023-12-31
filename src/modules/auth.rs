use actix_web::{web, Scope};

mod controller;
pub mod model;
pub mod service;

pub fn get_auth_routes() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(controller::login))
}
