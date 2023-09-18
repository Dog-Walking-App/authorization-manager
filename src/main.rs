use actix_web::{web, App, HttpServer};
use env::EnvVars;
use std::sync::Mutex;

use crate::args::Args;
mod args;
mod env;

use crate::utils::jwt::JWT;

use crate::modules::users;
use crate::modules::auth;
mod modules;

use crate::app_state::AppState;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod schema;
pub mod app_state;
pub mod utils;


pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = Args::get();
    let env_vars = EnvVars::load();

    let connection = establish_connection(&env_vars.database_url);

    let jwt = JWT::new(env_vars.jwt_secret);

    let app_state = web::Data::new(AppState {
        connection: Mutex::new(connection),
        jwt,
    });

    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(users::get_users_routes())
            .service(auth::get_auth_routes())
    })
    .bind((args.host, args.port))?
    .run()
    .await
}
