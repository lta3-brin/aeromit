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
    delete,
    HttpResponse,
};
use crate::app::{
    dto::UmpanBalik,
    errors::AppErrors,
};
use crate::pengguna::services::delete_user;


/// # Fungsi by_id
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil hapus pengguna sesuai id
/// saat mengunjungi _endpoint root_ `v1/pengguna`.
///
/// <br />
///
/// # Masukan
///
/// * `id` - id dokumen yang ingin ditelusuri.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[delete("/pengguna/{id}/")]
pub async fn by_id(
    id: web::Path<String>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    let count = delete_user::by_id(id, db).await?;

    if count == 0 {
        Ok(HttpResponse::NotFound().json(UmpanBalik::<i64> {
            sukses: false,
            pesan: "Pengguna tidak ditemukan".to_string(),
            hasil: count,
        }))
    } else {
        Ok(HttpResponse::Ok().json(UmpanBalik::<i64> {
            sukses: true,
            pesan: "Pengguna berhasil dihapus".to_string(),
            hasil: count,
        }))
    }
}
