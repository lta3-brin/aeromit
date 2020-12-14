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
use actix_session::Session;
use actix_web::{
    web,
    HttpResponse,
};
use crate::app::errors::AppErrors;
use crate::app::dto::UmpanBalik;
use crate::pengguna::{
    services::update_user,
    dto::UbahPenggunaDto,
};
use crate::app::permissions::UserPermissions;


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
/// * `payload` - inputan dari pengguna dalam bentuk `Form`.
/// * `session` - Actix session
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
pub async fn save(
    id: web::Path<String>,
    payload: web::Form<UbahPenggunaDto>,
    session: Session,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(session, db.clone()).await?;

    let count = update_user::save(id, payload, db).await?;

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
            "Pengguna berhasil disimpan",
            count
        );

        Ok(HttpResponse::Ok().json(res))
    }
}
