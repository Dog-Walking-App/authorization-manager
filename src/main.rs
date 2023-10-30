use actix_web::{web, App, HttpServer};
use env::EnvVars;
use std::sync::{Arc, Mutex};

use crate::args::Args;
mod args;
mod env;

use crate::utils::jwt::JWT;

use crate::modules::users;
use crate::modules::users::service::UsersService;
use crate::modules::users::user_jwt::UserJWT;
use crate::modules::auth;
use crate::modules::auth::service::AuthService;
mod modules;

use crate::app_state::AppState;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod schema;
pub mod app_state;
pub mod utils;

pub struct DBConfig {
    username: String,
    password: String,
    host: String,
    port: String,
    name: String,
}

pub fn establish_connection(config: &DBConfig) -> PgConnection {
    let database_url = format!("postgres://{}:{}@{}:{}/{}", config.username, config.password, config.host, config.port, config.name);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|error| panic!("Error connecting to DB: {}", error))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = Args::get();
    let env_vars = EnvVars::load();

    let connection = establish_connection(&DBConfig {
        username: env_vars.db_username,
        password: env_vars.db_password,
        host: env_vars.db_host,
        port: env_vars.db_port,
        name: env_vars.db_name,
    });

    let connection_mutex = Arc::new(Mutex::new(connection));

    let jwt = JWT::new(env_vars.jwt_secret);
    
    let user_jwt = UserJWT::new(jwt.clone());

    let users_service = UsersService::new(connection_mutex.clone());
    let auth_service = AuthService::new(connection_mutex.clone(), &jwt);

    let app_state = web::Data::new(AppState {
        users_service: Mutex::new(users_service),
        auth_service: Mutex::new(auth_service),
        user_jwt,
        jwt: jwt.clone(),
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
