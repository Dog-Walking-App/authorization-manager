use actix_web::{
    HttpRequest,
    http::header::HeaderValue,
};

pub struct ReqAuthorization {}

impl ReqAuthorization {
    pub fn get_token(req: &HttpRequest) -> Result<String, actix_web::Error> {
        let authorization = Self::extract_authorization_header(req)?;
        Self::extract_token(authorization)
    }

    fn extract_authorization_header(req: &HttpRequest) -> Result<Option<&HeaderValue>, actix_web::Error> {
        Ok(req.headers().get("Authorization"))
    }

    fn extract_token(authorization: Option<&HeaderValue>) -> Result<String, actix_web::Error> {
        let header_value = authorization
            .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?;
        let parts: Vec<&str> = header_value.to_str()
            .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Authorization header"))?
            .splitn(2, ' ')
            .collect();
        if parts.len() != 2 || parts[0] != "Bearer" {
            return Err(actix_web::error::ErrorUnauthorized("Invalid Authorization header"));
        }
        Ok(parts[1].to_owned())
    }
}
