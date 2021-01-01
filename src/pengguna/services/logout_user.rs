//! # Module User Logout Service
//!
//! Module ini digunakan untuk membersihkan session pengguna dan digunakan di `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::logout_user::{...}
//! ```
use crate::app::errors::AppErrors;


/// # Fungsi run
///
/// Fungsi ini untuk menjalankan fungsi hapus session.
///
/// <br />
///
/// # Masukan
///
/// * `session` - actix session.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<bool, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// `()` dan _Enum_ `AppErrors`.
pub async fn run() -> Result<(), AppErrors> {
    session.remove("masuk");

    Ok(())
}
