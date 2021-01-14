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
use actix_web::{get, HttpResponse, HttpRequest};
use crate::app::dto::UmpanBalik;
use crate::app::errors::AppErrors;
use crate::pengguna::services::check_user;
use crate::app::permissions::Roles;


/// # Fungsi is_authenticated
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil periksa peran pengguna
/// saat mengunjungi _endpoint root_ `v1/pengguna/check`.
///
/// <br />
///
/// # Masukan
///
/// * `req` - HttpRequest
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[get("pengguna/check/")]
pub async fn is_authenticated(req: HttpRequest) -> Result<HttpResponse, AppErrors> {
    let checked = check_user::run(req, Roles::Authenticated)?;

    if checked {
        let res = UmpanBalik::new(
            checked,
            "Pengguna terotentikasi",
            ()
        );

        Ok(HttpResponse::Accepted().json(res))
    } else {
        let res = UmpanBalik::new(
            checked,
            "Pengguna tidak terotentikasi",
            ()
        );

        Ok(HttpResponse::Unauthorized().json(res))
    }
}

/// # Fungsi is_admin
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil periksa peran pengguna
/// saat mengunjungi _endpoint root_ `v1/pengguna/adminkah`.
///
/// <br />
///
/// # Masukan
///
/// * `req` - HttpRequest
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[get("pengguna/adminkah/")]
pub async fn is_admin(req: HttpRequest) -> Result<HttpResponse, AppErrors> {
    let checked = check_user::run(req, Roles::Admin)?;

    if checked {
        let res = UmpanBalik::new(
            checked,
            "Pengguna admin",
            ()
        );

        Ok(HttpResponse::Accepted().json(res))
    } else {
        let res = UmpanBalik::new(
            checked,
            "Pengguna bukan admin",
            ()
        );

        Ok(HttpResponse::Unauthorized().json(res))
    }
}
