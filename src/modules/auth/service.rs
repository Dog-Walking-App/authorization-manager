use std::sync::{Arc, Mutex};
use diesel::prelude::*;
use crate::utils::jwt::JWT;
use crate::utils::password::verify_password;
use crate::utils::service_error::ServiceError;
use super::super::users::model::User;
use super::super::users::db::{UsersDB, UsersDBTrait};
use super::super::users::user_jwt::UserJWT;
use super::model::Credentials;

pub struct LoginOutput {
    pub jwt: String,
    pub user: User,
}

pub trait AuthServiceTrait {
    fn login(&mut self, credentials: &Credentials) -> Result<LoginOutput, ServiceError>;
}

pub struct AuthService {
    users_db: UsersDB,
    user_jwt: UserJWT,
}

impl<'a> AuthService {
    pub fn new(
        connection: Arc<Mutex<PgConnection>>,
        jwt: &'a JWT,
    ) -> AuthService {
        let user_jwt = UserJWT::new(jwt.clone());
        let users_db = UsersDB::new(connection);
        AuthService { users_db, user_jwt }
    }
}

impl<'a> AuthServiceTrait for AuthService {
    fn login(&mut self, credentials: &Credentials) -> Result<LoginOutput, ServiceError> {
        let user = self.users_db.find_by_username(&credentials.username)
            .map_err(|_| ServiceError::new("Failed to find the user"))?;

        let is_valid = verify_password(
            &credentials.password,
            &user.password
        ).map_err(|_| ServiceError::new("Failed to verify password"))?;

        if is_valid {
            Ok(LoginOutput {
                jwt: self.user_jwt.generate(&user),
                user
            })
        } else {
            Err(ServiceError::new("Invalid password"))
        }
    }
}
