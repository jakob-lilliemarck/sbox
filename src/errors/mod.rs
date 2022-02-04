use actix_web::{http::StatusCode, HttpResponse};
use derive_more::Display;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse<'a> {
    error: Option<&'a str>,
}

#[derive(Debug, Display)]
pub enum ServerError<'a> {
    #[display(fmt = "Not found")]
    NotFound,
    #[display(fmt = "Bad request")]
    BadRequest(Option<&'a str>),
    #[display(fmt = "Unknown error")]
    Unknown,
}

impl<'a> ServerError<'a> {
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::NotFound => Some("Not found"),
            Self::BadRequest(cause) => match cause {
                Some(cause) => Some(cause),
                None => Some("Bad request"),
            },
            Self::Unknown => Some("Unknown error"),
        }
    }
}

impl<'a> From<diesel::result::Error> for ServerError<'a> {
    fn from(err: diesel::result::Error) -> ServerError<'a> {
        match err {
            diesel::result::Error::NotFound => ServerError::NotFound,
            _ => ServerError::Unknown,
        }
    }
}

impl<'a> actix_web::error::ResponseError for ServerError<'a> {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_cause) => StatusCode::BAD_REQUEST,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.message(),
        })
    }
}
