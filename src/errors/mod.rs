use actix_web::{http::StatusCode, HttpResponse};
use actix_web_httpauth::extractors::AuthenticationError;
use derive_more::Display;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse<'a> {
    error: Option<&'a str>,
}

#[derive(Debug, Display, Clone)]
pub enum ServerError<'a> {
    #[display(fmt = "Not found")]
    NotFound,
    #[display(fmt = "Bad request")]
    BadRequest(Option<&'a str>),
    #[display(fmt = "Forbidden")]
    Forbidden(Option<&'a str>),
    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
    #[display(fmt = "Unauthorized")]
    Unauthorized,
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
            Self::Forbidden(cause) => match cause {
                Some(cause) => Some(cause),
                None => Some("Forbidden"),
            },
            Self::Unknown => Some("Unknown error"),
            Self::JWKSFetchError => Some("JWKS error"),
            Self::Unauthorized => Some("Unauthorized"),
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

impl<'a> From<Box<dyn std::error::Error>> for ServerError<'a> {
    fn from(err: Box<dyn std::error::Error>) -> ServerError<'a> {
        match err {
            _ => ServerError::Unknown,
        }
    }
}

impl<'a> actix_web::error::ResponseError for ServerError<'a> {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_cause) => StatusCode::BAD_REQUEST,
            Self::Forbidden(_cause) => StatusCode::FORBIDDEN,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::JWKSFetchError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.message(),
        })
    }
}
