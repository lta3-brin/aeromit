//! # Module Update User Handler
//!
//! Module ini digunakan untuk ubah pengguna berdasarkan id sebagai `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::update_user::{...}
//! ```
use mongodb::Database;
use actix_web::{
    web,
    put,
    HttpResponse,
};
use crate::app::errors::AppErrors;
use crate::app::dto::UmpanBalik;
use crate::pengguna::{
    services::update_user,
    dto::UbahPenggunaDto,
};


/// # Fungsi save
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
#[put("/pengguna/{id}/")]
pub async fn save(
    id: web::Path<String>,
    payload: web::Form<UbahPenggunaDto>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    update_user::save(id, payload, db).await?;

    Ok(HttpResponse::Ok().json(UmpanBalik::<()> {
        sukses: true,
        pesan: "Pengguna berhasil disimpan".to_string(),
        hasil: (),
    }))
}