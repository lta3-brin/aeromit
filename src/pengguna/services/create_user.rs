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
use mongodb::Database;
use crate::app::errors::AppErrors;
use crate::pengguna::{
    dto::PenggunaDto,
    helpers::{PenggunaHelpers, PenggunaHelpersTrait},
};


/// # Fungsi new
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
    let collection = db.collection("pengguna");

    payload.validate()?;

    let dok = <PenggunaHelpers as PenggunaHelpersTrait>::create_to_doc(payload)?;

    collection
        .insert_one(dok,None)
        .await?;

    Ok(())
}
