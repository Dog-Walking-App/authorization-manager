use diesel::prelude::*;
use crate::schema::users::dsl::*;
use crate::utils::jwt::JWT;
use crate::utils::password::verify_password;
use crate::utils::service_error::ServiceError;
use super::super::users::model::User;
use super::super::users::user_jwt::UserJWT;
use super::model::Credentials;

pub struct LoginOutput {
    pub jwt: String,
    pub user: User,
}

pub trait AuthServiceTrait {
    fn login(&mut self, credentials: &Credentials) -> Result<LoginOutput, ServiceError>;
}

pub struct AuthService<'a> {
    connection: &'a mut PgConnection,
    user_jwt: UserJWT<'a>,
}

impl<'a> AuthService<'a> {
    pub fn new(
        connection: &'a mut PgConnection,
        jwt: &'a JWT,
    ) -> AuthService<'a> {
        let user_jwt = UserJWT::new(jwt);
        AuthService { connection, user_jwt }
    }
}

fn get_user_by_username(
    connection: &mut PgConnection,
    value: &str,
) -> QueryResult<User> {
    users
        .filter(username.eq(value))
        .load::<User>(connection)?
        .pop()
        .ok_or(diesel::result::Error::NotFound)
}

impl<'a> AuthServiceTrait for AuthService<'a> {
    fn login(&mut self, credentials: &Credentials) -> Result<LoginOutput, ServiceError> {
        let user = get_user_by_username(
            self.connection,
            &credentials.username
        ).map_err(|error| ServiceError::new(&error.to_string()))?;

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
