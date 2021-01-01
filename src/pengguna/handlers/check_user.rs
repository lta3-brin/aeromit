//! # Module Check User Handler
//!
//! Module ini digunakan untuk periksa pengguna apakah session masih valid
//! sebagai `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::check_user::{...}
//! ```
use actix_web::{get, HttpResponse};
use crate::app::dto::UmpanBalik;
use crate::app::errors::AppErrors;
use crate::pengguna::services::check_user;


/// # Fungsi is_ok
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil periksa session pengguna
/// saat mengunjungi _endpoint root_ `v1/pengguna/check`.
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
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[get("pengguna/check/")]
pub async fn is_ok() -> Result<HttpResponse, AppErrors> {
    let checked = check_user::run()?;

    if checked {
        let res = UmpanBalik::new(
            checked,
            "Pengguna dicek",
            ()
        );

        Ok(HttpResponse::Accepted().json(res))
    } else {
        let res = UmpanBalik::new(
            checked,
            "Pengguna dicek",
            ()
        );

        Ok(HttpResponse::Unauthorized().json(res))
    }
}
