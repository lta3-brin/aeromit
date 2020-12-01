//! # Module Create User Service
//!
//! Module ini digunakan untuk membuat atau daftar pengguna baru untuk digunakan di `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::create_user::{...}
//! ```
use actix_web::web;
use validator::Validate;
use mongodb::{Database, bson::doc};
use crate::app::errors::AppErrors;
use crate::pengguna::dto::PenggunaDto;
use chrono::Utc;


/// # Fungsi tambah_pengguna_service
///
/// Fungsi ini untuk menambahkan data `Pengguna` baru.
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
/// * `Result<(), AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari ()
/// dan _Enum_ `AppErrors`.
pub async fn new(
    payload: web::Form<PenggunaDto>,
    db: web::Data<Database>
) -> Result<(), AppErrors> {
    let admin: bool;
    let collection = db.collection("pengguna");

    payload.validate()?;

    if payload.0.isadmin == 1 {
        admin = true
    } else { admin = false }

    collection
        .insert_one(
            doc! {
                "nama": payload.0.nama,
                "email": payload.0.email,
                "password": payload.0.password,
                "isadmin": admin,
                "dibuat": Utc::now(),
            },
            None
        )
        .await?;

    Ok(())
}