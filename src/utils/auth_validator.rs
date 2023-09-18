use actix_web::{web, dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use crate::app_state::AppState;

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let app_data = req.app_data::<web::Data<AppState>>().unwrap();

    match app_data.jwt.validate(credentials.token()) {
        Ok(_) => Ok(req),
        Err(_) => {
            let config = req.app_data::<Config>()
                .cloned()
                .unwrap_or_default();

            Err((AuthenticationError::from(config).into(), req))
        },
    }
}
