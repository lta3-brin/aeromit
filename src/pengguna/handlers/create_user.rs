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
use actix_session::Session;
use actix_web::{
    web,
    HttpResponse,
};
use crate::app::dto::UmpanBalik;
use crate::app::errors::AppErrors;
use crate::pengguna::{
    dto::PenggunaDto,
    services::create_user,
};
use crate::app::permissions::UserPermissions;


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
/// * `session` - Actix session
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
pub async fn new(
    payload: web::Form<PenggunaDto>,
    session: Session,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(session, db.clone()).await?;

    create_user::new(payload, db).await?;

    let res = UmpanBalik::new(
        true,
        "Pengguna berhasil ditambahkan",
        ()
    );

    Ok(HttpResponse::Created().json(res))
}
