use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct EnvVars {
    // Database URL for connecting to the database
    pub database_url: String,

    // Secret key for generating JWT tokens
    pub jwt_secret: String,
}

impl EnvVars {
    pub fn load() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        EnvVars { database_url, jwt_secret }
    }
}