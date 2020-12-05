//! # Module Create User Handler
//!
//! Module ini digunakan untuk membuat pengguna baru sebagai `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::create_user::{...}
//! ```
use mongodb::Database;
use actix_web::{
    web,
    post,
    HttpResponse,
};
use crate::app::dto::UmpanBalik;
use crate::app::errors::AppErrors;
use crate::pengguna::{
    dto::PenggunaDto,
    services::create_user,
};


/// # Fungsi new
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil penambahan pengguna baru
/// saat mengunjungi _endpoint root_ `v1/pengguna`.
///
/// <br />
///
/// # Masukan
///
/// * `payload` - Data masukan dari pengguna untuk tambah pengguna.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[post("/pengguna/")]
pub async fn new(
    payload: web::Form<PenggunaDto>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    create_user::new(payload, db).await?;

    Ok(HttpResponse::Created().json(UmpanBalik::<()> {
        sukses: true,
        pesan: "Pengguna berhasil ditambahkan".to_string(),
        hasil: (),
    }))
}
