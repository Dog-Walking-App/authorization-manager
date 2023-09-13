use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

use crate::args::Args;
mod args;

use crate::modules::users;
use crate::modules::auth;
mod modules;

use crate::app_state::AppState;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod schema;
pub mod app_state;
pub mod utils;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = Args::get();

    let connection = establish_connection();
    let app_state = web::Data::new(AppState {
        connection: Mutex::new(connection),
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
