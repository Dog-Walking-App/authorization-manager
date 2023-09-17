use std::sync::Mutex;
use diesel::pg::PgConnection;

use crate::utils::jwt::JWT;

pub struct AppState {
  pub connection: Mutex<PgConnection>, // <- Mutex is necessary to mutate safely across threads
  pub jwt: JWT,
}