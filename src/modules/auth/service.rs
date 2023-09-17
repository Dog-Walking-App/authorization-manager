use diesel::prelude::*;
use crate::schema::users::dsl::*;
use crate::utils::jwt::JWT;
use crate::utils::password::verify_password;
use super::super::users::model::User;
use super::super::users::user_jwt::UserJWT;
use super::model::Credentials;

pub struct LoginOutput {
    pub jwt: String,
    pub user: User,
}

pub trait AuthServiceTrait {
    fn login(&mut self, credentials: &Credentials) -> QueryResult<LoginOutput>;
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

impl<'a> AuthServiceTrait for AuthService<'a> {
    fn login(&mut self, credentials: &Credentials) -> QueryResult<LoginOutput> {
        let user = users
            .filter(username.eq(&credentials.username))
            .load::<User>(self.connection)?
            .pop()
            .ok_or(diesel::result::Error::NotFound)?;

        let is_valid = verify_password(
            &credentials.password,
            &user.password
        ).expect("Failed to verify password");

        if is_valid {
            Ok(LoginOutput {
                jwt: self.user_jwt.generate(&user),
                user
            })
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }
}
