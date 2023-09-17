use actix_web::{web, Scope};

pub mod controller;
pub mod model;
pub mod service;
pub mod user_jwt;

pub fn get_users_routes() -> Scope {
    web::scope("/users")
        .service(controller::get_users)
        .service(controller::get_me)
        .service(controller::get_user_by_id)
        .service(controller::create_user)
}
