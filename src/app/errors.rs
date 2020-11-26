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
use crate::app::dto::UmpanBalik;


/// Enum AppErrors digunakan sebagai tempat untuk mengambil jenis kesalahan yang berbeda
/// yang mungkin terjadi dalam aplikasi
#[derive(Debug, Display, Error)]
pub enum AppErrors {
    EnvError(env::VarError),
    MongoError(mongodb::error::Error),
    ActixError(io::Error),
    BsonAccessError(bson::document::ValueAccessError),
    BsonDeserializeError(bson::de::Error),
    ParseOidError(bson::oid::Error)
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

/// Menampilkan kesalahan yang mungkin terjadi dari `AppErrors` kedalam bentuk `ResponseError`
impl ResponseError for AppErrors {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppErrors::ParseOidError(..) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .json(UmpanBalik::<String> {
                sukses: false,
                pesan: "Terjadi kesalahan yang perlu diperhatikan".to_string(),
                hasil: self.to_string()
            })
    }
}
