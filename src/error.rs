//! Main Crate Error

use actix_web::{http::StatusCode, ResponseError};

// impl ResponseError for W<sqlx::Error> {
//     fn status_code(&self) -> actix_web::http::StatusCode {
//         actix_web::http::StatusCode::FAILED_DEPENDENCY
//     }
// }

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // /// For starter, to remove as code matures.
    // #[error("Static error: {0}")]
    // Static(&'static str),
    #[error("Actix web error: {0}")]
    Actix(#[from] actix_web::error::Error),
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Serde json error: {0}")]
    Serde(#[from] serde_json::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use crate::Error::*;
        match self {
            Serde(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
