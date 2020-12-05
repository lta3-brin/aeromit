//! # Module Update User By Id Service
//!
//! Module ini digunakan untuk ubah data pengguna berdasarkan id untuk digunakan di `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::update_user::{...}
//! ```
use actix_web::web;
use validator::Validate;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId}
};
use crate::app::errors::AppErrors;
use crate::pengguna::{
    helpers::{PenggunaHelpersTrait, PenggunaHelpers},
    dto::UbahPenggunaDto
};


/// # Fungsi save
///
/// Fungsi ini untuk simpan ubahan data `Pengguna` sesuai id.
///
/// <br />
///
/// # Masukan
///
/// * `uid` - id unik dokumen untuk dipilih.
/// * `payload` - Data masukan dari pengguna untuk ubah data.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<(), AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// `()` dan _Enum_ `AppErrors`.
pub async fn save(
    uid: web::Path<String>,
    payload: web::Form<UbahPenggunaDto>,
    db: web::Data<Database>
) -> Result<(), AppErrors> {
    let id = ObjectId::with_string(uid.trim())?;
    let collection = db.collection("pengguna");

    payload.validate()?;

    let dok = <PenggunaHelpers as PenggunaHelpersTrait>::update_to_doc(payload)?;

    collection
        .update_one(doc! {"_id": id}, dok, None)
        .await?;

    Ok(())
}
