use std::sync::Mutex;
use diesel::pg::PgConnection;

pub struct AppState {
  pub connection: Mutex<PgConnection>, // <- Mutex is necessary to mutate safely across threads
}