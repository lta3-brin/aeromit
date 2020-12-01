//! # Module Handlers
//!
//! Module ini digunakan sebagai _Controller_ atau penggerak atau pengelola berupa
//! fungsi-fungsi _request_/_response_ server.
//!
//! Fungsi-fungsi dari module handlers ini biasa digunakan didalam module `routes`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::{...}
//! ```
use mongodb::Database;
use actix_web::{
    web,
    post,
    HttpResponse,
};
use crate::app::errors::AppErrors;
use crate::app::dto::UmpanBalik;
use crate::pengguna::dto::PenggunaDto;
use crate::pengguna::services::create_user;


#[post("/pengguna/")]
pub async fn tambah_pengguna_handler(
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
