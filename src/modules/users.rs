use actix_web::{web, Scope};

pub mod model;

mod controller;
mod service;


pub fn get_users_routes() -> Scope {
    web::scope("/users")
        .service(controller::get_users)
        .service(controller::get_user_by_id)
}
