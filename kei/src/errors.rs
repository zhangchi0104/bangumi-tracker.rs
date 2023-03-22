use std::fmt::Debug;
use std::sync::PoisonError;

use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Unable to serialize to {0}")]
    SerializeError(&'static str),
    #[error("Unable to deserialize to {0}")]
    DeserializeError(&'static str),
    #[error("Mutex lock failed")]
    PoisonError,
    #[error("API request failed: {0:?}")]
    ApiClientError(Option<&'static str>),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_value: PoisonError<T>) -> Self {
        Self::PoisonError
    }
}

impl Error {
    pub fn error_code(&self) -> u8 {
        match self {
            Error::SerializeError(_) => 0x1,
            Error::DeserializeError(_) => 0x2,
            Error::PoisonError => 0x3,
            Error::ApiClientError(_) => 0x4,
        }
    }

    pub fn payload(&self) -> serde_json::Value {
        match self {
            Error::SerializeError(typename) => json!({ "type": typename }),
            Error::DeserializeError(typename) => json!({ "type": typename }),
            Error::PoisonError => serde_json::Value::Null,
            Error::ApiClientError(url) => json!({ "url": url }),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::SerializeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DeserializeError(_) => StatusCode::BAD_REQUEST,
            Error::PoisonError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ApiClientError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let errcode = self.error_code();
        let payload = self.payload();

        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .json(json!({
                "errCode": errcode,
                "payload": payload,
            }))
    }
}

pub trait LogError<T, E> {
    fn map_logged_err<U, F>(self, op: F) -> Result<T, U>
    where
        F: FnOnce(E) -> U;
    fn log_err(self) -> Result<T, E>;
}

impl<T, E> LogError<T, E> for std::result::Result<T, E>
where
    E: Debug,
{
    fn map_logged_err<U, F>(self, op: F) -> Result<T, U>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                log::error!("{e:?}");
                Err(op(e))
            }
        }
    }

    fn log_err(self) -> Result<T, E> {
        if let Err(e) = &self {
            log::error!("{e:?}");
        }
        self
    }
}
