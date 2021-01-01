//! # Module Check User Service
//!
//! Module ini digunakan untuk periksa pengguna apakah session masih valid
//! dan digunakan di dalam `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::check_user::{...}
//! ```
use actix_web::HttpRequest;
use crate::app::errors::AppErrors;
use crate::app::helpers::{AppHelpers, AppHelpersTrait};


/// # Fungsi run
///
/// Fungsi ini untuk menjalankan fungsi periksa session user.
///
/// <br />
///
/// # Masukan
///
/// * `req` - Actix Http Request
///
/// <br />
///
/// # Keluaran
///
/// * `Result<bool, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `bool` dan _Enum_ `AppErrors`.
pub fn run(req: HttpRequest) -> Result<bool, AppErrors> {
    let headers = req.headers().get("authorization");
    let token = <AppHelpers as AppHelpersTrait>::get_token(headers)?;

    if token.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}
