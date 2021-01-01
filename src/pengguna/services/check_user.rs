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
use crate::app::errors::AppErrors;


/// # Fungsi run
///
/// Fungsi ini untuk menjalankan fungsi periksa session user.
///
/// <br />
///
/// # Masukan
///
/// * `session` - Actix session
///
/// <br />
///
/// # Keluaran
///
/// * `Result<bool, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `bool` dan _Enum_ `AppErrors`.
pub fn run() -> Result<bool, AppErrors> {
    let token = session.get::<String>("masuk")?;

    if token.is_none() {
        Ok(false)
    } else {
        Ok(true)
    }
}
