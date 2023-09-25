use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ResponseErrorJson {
    message: String,
}

#[derive(Debug, Display, Error)]
pub enum ResponseError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },

    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,

    #[display(fmt = "{} not found.", entity)]
    ErrorNotFound { entity: String },

    #[display(fmt = "{}", message)]
    ErrorUnauthorized { message: String },
}

impl error::ResponseError for ResponseError {
    fn error_response(&self) -> HttpResponse {
        let error_message = ResponseErrorJson { message: self.to_string() };
        let json_error_message = serde_json::to_string(&error_message).unwrap();

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(json_error_message)
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            ResponseError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            ResponseError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::ErrorNotFound { .. } => StatusCode::NOT_FOUND,
            ResponseError::ErrorUnauthorized { .. } => StatusCode::UNAUTHORIZED,
        }
    }
}
