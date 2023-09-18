use std::time::{SystemTime, Duration};
use serde::{Serialize, Deserialize};
use crate::utils::jwt::JWT;
use super::super::users::model::User;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub user_id: i32,
    pub username: String,
    pub exp: usize,
}

pub struct UserJWT<'a> {
    jwt: &'a JWT,
}

impl<'a> UserJWT<'a> {
    pub fn new(jwt: &'a JWT) -> UserJWT<'a> {
        UserJWT { jwt }
    }
    
    pub fn generate(&self, user: &User) -> String {
        let expiration_time = SystemTime::now() + Duration::from_secs(60 * 60 * 24);
        let duration = expiration_time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let expiration_timestamp = duration.as_secs() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.to_owned(),
            exp: expiration_timestamp,
        };
        self.jwt.generate(&claims)
    }

    pub fn get_claims(
        &self,
        token: &String,
    ) -> Result<UserClaims, String> {
        let claims = self.jwt.get_claims::<Claims>(token)?;
        let user_id = claims.sub.parse::<i32>().unwrap();

        Ok(UserClaims {
            user_id,
            username: claims.username,
            exp: claims.exp,
        })
    }
}