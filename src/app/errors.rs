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

/// Enum AppErrors digunakan sebagai tempat untuk mengambil jenis kesalahan yang berbeda
/// yang mungkin terjadi dalam aplikasi
#[derive(Debug)]
pub enum AppErrors {
    EnvError(env::VarError),
    MongoError(mongodb::error::Error),
    ActixError(io::Error)
}

impl From<env::VarError> for AppErrors {
    fn from(err: env::VarError) -> Self {
        Self::EnvError(err)
    }
}

impl From<mongodb::error::Error> for AppErrors {
    fn from(err: mongodb::error::Error) -> Self {
        Self::MongoError(err)
    }
}

impl From<io::Error> for AppErrors {
    fn from(err: io::Error) -> Self {
        Self::ActixError(err)
    }
}
