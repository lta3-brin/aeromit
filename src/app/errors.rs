//! # Module Errors
//!
//! Module ini digunakan untuk mengatasi `errors` yang mungkin terjadi
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::app::errors::{...}
//! ```
use std::{env, io};
use derive_more::{Display, Error};
use mongodb::bson;
use actix_web::{
    error::ResponseError,
    HttpResponse,
    dev::HttpResponseBuilder,
    http::{header, StatusCode}
};
use std::num::ParseIntError;
use crate::app::dto::UmpanBalik;


/// Enum AppErrors digunakan sebagai tempat untuk mengambil jenis kesalahan yang berbeda
/// yang mungkin terjadi dalam aplikasi
#[derive(Debug, Display, Error)]
pub enum AppErrors {
    EnvError(env::VarError),
    MongoError(mongodb::error::Error),
    ActixError(io::Error),
    ActixWebError(actix_web::Error),
    BsonAccessError(bson::document::ValueAccessError),
    BsonDeserializeError(bson::de::Error),
    ParseIntegerError(ParseIntError),
    ParseOidError(bson::oid::Error),
    ParseChronoError(chrono::ParseError),
    InputValidationError(validator::ValidationErrors),
    HashingError(argon2::Error),
    JwtError(jsonwebtoken::errors::Error),
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `env::VarError`
impl From<env::VarError> for AppErrors {
    fn from(err: env::VarError) -> Self {
        Self::EnvError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `mongodb::error::Error`
impl From<mongodb::error::Error> for AppErrors {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `io::Error`
impl From<io::Error> for AppErrors {
    fn from(err: io::Error) -> Self {
        Self::ActixError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `actix_web::Error`
impl From<actix_web::Error> for AppErrors {
    fn from(err: actix_web::Error) -> Self {
        Self::ActixWebError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `bson::document::ValueAccessError`
impl From<bson::document::ValueAccessError> for AppErrors {
    fn from(err: bson::document::ValueAccessError) -> Self {
        Self::BsonAccessError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `bson::de::Error`
impl From<bson::de::Error> for AppErrors {
    fn from(err: bson::de::Error) -> Self {
        Self::BsonDeserializeError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `bson::oid::Error`
impl From<bson::oid::Error> for AppErrors {
    fn from(err: bson::oid::Error) -> Self {
        Self::ParseOidError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `chrono::ParseError`
impl From<chrono::ParseError> for AppErrors {
    fn from(err: chrono::ParseError) -> Self {
        Self::ParseChronoError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `chrono::ParseError`
impl From<validator::ValidationErrors> for AppErrors {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::InputValidationError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `chrono::ParseError`
impl From<argon2::Error> for AppErrors {
    fn from(err: argon2::Error) -> Self {
        Self::HashingError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `jsonwebtoken::errors::Error`
impl From<jsonwebtoken::errors::Error> for AppErrors {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::JwtError(err)
    }
}

/// Implementasi `AppErrors` apabila terjadi kesalahan seputar `ParseIntError`
impl From<ParseIntError> for AppErrors {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntegerError(err)
    }
}

/// Menampilkan kesalahan yang mungkin terjadi dari `AppErrors` kedalam bentuk `ResponseError`
impl ResponseError for AppErrors {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppErrors::ParseOidError(..) => StatusCode::BAD_REQUEST,
            AppErrors::ParseChronoError(..) => StatusCode::BAD_REQUEST,
            AppErrors::InputValidationError(..) => StatusCode::BAD_REQUEST,
            AppErrors::ActixWebError(..) => StatusCode::FORBIDDEN,
            AppErrors::JwtError(..) => StatusCode::NOT_ACCEPTABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let res = UmpanBalik::new(
            false,
            "Terjadi kesalahan yang perlu diperhatikan",
            self.to_string()
        );

        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .json(res)
    }
}
