//! # Module Logout User Handler
//!
//! Module ini digunakan untuk logout pengguna sebagai `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::logout_user::{...}
//! ```
use actix_web::{
    post,
    HttpResponse,
};
use actix_session::Session;
use crate::app::errors::AppErrors;
use crate::app::dto::UmpanBalik;
use crate::pengguna::services::logout_user;


/// # Fungsi keluar
///
/// Fungsi ini untuk menampilkan _response_ umpan balik setelah pengguna keluar sesuai inputan
/// pengguna saat mengunjungi _endpoint root_ `v1/pengguna/logout`.
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
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[post("/pengguna/logout/")]
pub async fn keluar(session: Session) -> Result<HttpResponse, AppErrors> {
    logout_user::run(session).await?;

    let res = UmpanBalik::new(
        true,
        "Pengguna berhasil keluar",
        ()
    );

    Ok(HttpResponse::Ok().json(res))
}
