use diesel::prelude::*;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use crate::schema::users::dsl::*;
use crate::utils::password::verify_password;
use super::super::users::model::User;
use super::model::Credentials;


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    username: String,
    exp: usize,
}

fn generate_jwt(secret: &str, user: &User, exp: usize) -> String {
    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.to_owned(),
        exp,
    };
    let header = Header::default();
    let key = EncodingKey::from_secret(secret.as_ref());
    let token = encode(&header, &claims, &key).unwrap();
    token
}

pub struct LoginOutput {
    pub jwt: String,
    pub user: User,
}

pub trait AuthServiceTrait {
    fn login(&mut self, credentials: &Credentials) -> QueryResult<LoginOutput>;
}

pub struct AuthService<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> AuthService<'a> {
    pub fn new(connection: &'a mut PgConnection) -> AuthService<'a> {
        AuthService { connection }
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
                jwt: generate_jwt("secret", &user, 100000),
                user
            })
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }
}
