use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct EnvVars {
    // Database username, password, host, port, and name
    pub db_username: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,

    // Secret key for generating JWT tokens
    pub jwt_secret: String,
}

impl EnvVars {
    pub fn load() -> Self {
        dotenv().ok();

        let db_username = env::var("DB_USERNAME")
            .expect("DB_USERNAME must be set");
        let db_password = env::var("DB_PASSWORD")
            .expect("DB_PASSWORD must be set");
        let db_host = env::var("DB_HOST")
            .expect("DB_HOST must be set");
        let db_port = env::var("DB_PORT")
            .expect("DB_PORT must be set");
        let db_name = env::var("DB_NAME")
            .expect("DB_NAME must be set");

        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");

        EnvVars {
            db_username,
            db_password,
            db_host,
            db_port,
            db_name,
            jwt_secret,
        }
    }
}