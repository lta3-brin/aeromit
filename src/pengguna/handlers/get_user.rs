//! # Module Get User Handler
//!
//! Module ini digunakan untuk ambil pengguna berdasarkan id sebagai `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::get_user::{...}
//! ```
use mongodb::Database;
use actix_web::{web, HttpResponse, HttpRequest};
use crate::app::errors::AppErrors;
use crate::app::dto::UmpanBalik;
use crate::pengguna::services::get_user;
use crate::app::permissions::UserPermissions;


/// # Fungsi by_id
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil baca pengguna sesuai id
/// saat mengunjungi _endpoint root_ `v1/pengguna/{id}`.
///
/// <br />
///
/// # Masukan
///
/// * `id` - id dokumen yang ingin ditelusuri.
/// * `session` - Actix session
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
pub async fn by_id(
    id: web::Path<String>,
    req: HttpRequest,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(req, db.clone()).await?;
    let pengguna_tertentu = get_user::by_id(id, db).await?;

    if pengguna_tertentu.is_none() {
        let res = UmpanBalik::new(
            false,
            "Pengguna tidak ditemukan",
            pengguna_tertentu
        );

        Ok(HttpResponse::NotFound().json(res))
    } else {
        let res = UmpanBalik::new(
            true,
            "Pengguna berhasil ditampilkan",
            pengguna_tertentu
        );

        Ok(HttpResponse::Ok().json(res))
    }
}
