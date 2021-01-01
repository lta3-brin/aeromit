//! # Module Delete User Handler
//!
//! Module ini digunakan untuk hapus pengguna berdasarkan id sebagai `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::delete_user::{...}
//! ```
use mongodb::Database;
use actix_web::{
    web,
    HttpResponse,
};
use crate::app::{
    dto::UmpanBalik,
    errors::AppErrors,
};
use crate::pengguna::services::delete_user;
use crate::app::permissions::UserPermissions;


/// # Fungsi by_id
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil hapus pengguna sesuai id
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
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(db.clone()).await?;

    let count = delete_user::by_id(id, db).await?;

    if count == 0 {
        let res = UmpanBalik::new(
            false,
            "Pengguna tidak ditemukan",
            count
        );

        Ok(HttpResponse::NotFound().json(res))
    } else {
        let res = UmpanBalik::new(
            true,
            "Pengguna berhasil dihapus",
            count
        );

        Ok(HttpResponse::Ok().json(res))
    }
}
