use std::sync::Mutex;
use crate::utils::jwt::JWT;
use crate::users::service::UsersService;
use crate::users::user_jwt::UserJWT;
use crate::auth::service::AuthService;

pub struct AppState {
  pub user_jwt: UserJWT,
  pub users_service: Mutex<UsersService>,
  pub auth_service: Mutex<AuthService>,
  pub jwt: JWT,
}