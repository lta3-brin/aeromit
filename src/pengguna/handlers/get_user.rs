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
use actix_web::{
    web,
    get,
    HttpResponse,
};
use crate::app::errors::AppErrors;
use crate::app::dto::UmpanBalik;
use crate::pengguna::{
    models::Pengguna,
    services::get_user,
};


/// # Fungsi by_id
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil baca pengguna sesuai id
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
#[get("/pengguna/{id}/")]
pub async fn by_id(
    id: web::Path<String>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    let pengguna_tertentu = get_user::by_id(id, db).await?;

    Ok(HttpResponse::Ok().json(UmpanBalik::<Option<Pengguna>> {
        sukses: true,
        pesan: "Pengguna berhasil ditampilkan".to_string(),
        hasil: pengguna_tertentu,
    }))
}
