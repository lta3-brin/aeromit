//! # Module Middlewares
//!
//! Module ini digunakan untuk bantuan sebagai middlewares.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::app::middles::{...}
//! ```
use actix_web::middleware::{self, Logger, NormalizePath};
use actix_cors::Cors;
use actix_session::CookieSession;


/// _Struct_ dijadikan sebagai kumpulan _middlewares_.
pub struct Middlewares;

impl Middlewares {
    /// Fungsi ini untuk menampilkan Log ke _console_ atau _terminal_
    pub fn build_logger() -> Logger {
        middleware::Logger::default()
    }

    /// Fungsi ini untuk kelola CORS
    pub fn set_cors() -> Cors {
        Cors::default()
            .allow_any_origin()
    }

    /// Fungsi ini untuk normalisasi url path
    pub fn normalize_path() -> NormalizePath {
        middleware::NormalizePath::default()
    }

    /// Fungsi ini untuk kelola _user session_.
    pub fn handle_session() -> CookieSession {
        CookieSession::signed(&[0; 32])
            .secure(false)
            .http_only(true)
            .name("aeromit")
    }
}
