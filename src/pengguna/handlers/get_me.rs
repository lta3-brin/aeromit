//! # Module Get Me Handler
//!
//! Module ini digunakan untuk ambil data pengguna sendiri berdasarkan token sebagai `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::get_me::{...}
//! ```
use mongodb::Database;
use actix_web::{web, HttpResponse, HttpRequest};
use crate::app::errors::AppErrors;
use crate::app::dto::UmpanBalik;
use crate::pengguna::services::get_me;
use crate::app::permissions::UserPermissions;


/// # Fungsi me_ok
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil baca pengguna sendiri sesuai token
/// yang dimiliki saat mengunjungi _endpoint root_ `v1/pengguna/me{id}`.
///
/// <br />
///
/// # Masukan
///
/// * `req` - HttpRequest
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
pub async fn me_ok(
    req: HttpRequest,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_authenticated(req.clone())?;
    let itsme = get_me::by_token(req, db).await?;

    let res = UmpanBalik::new(
        false,
        "Pengguna ditemukan",
        itsme
    );

    Ok(HttpResponse::Ok().json(res))
}
