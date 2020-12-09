//! # Module Get User By Id Service
//!
//! Module ini digunakan untuk membaca pengguna berdasarkan id untuk digunakan di `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::get_user::{...}
//! ```
use actix_web::web;
use mongodb::{
    Database,
    bson::{self, doc, document::Document, oid::ObjectId}
};
use crate::app::errors::AppErrors;
use crate::pengguna::{
    models::Pengguna,
    helpers::{PenggunaHelpersTrait, PenggunaHelpers},
};


/// # Fungsi by_id
///
/// Fungsi ini untuk melihat data `Pengguna` sesuai id.
///
/// <br />
///
/// # Masukan
///
/// * `uid` - id unik dokumen untuk dipilih.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<Option<Pengguna>, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// `Option<Pengguna>` dan _Enum_ `AppErrors`.
pub async fn by_id(
    uid: web::Path<String>,
    db: web::Data<Database>
) -> Result<Option<Pengguna>, AppErrors> {
    let id = ObjectId::with_string(uid.trim())?;
    let collection = db.collection("pengguna");
    let result = collection
        .find_one(doc! {"_id": id}, None)
        .await?;

    match result {
        Some(document) => {
            let dok = bson::from_document::<Document>(document)?;
            let peg = <PenggunaHelpers as PenggunaHelpersTrait>::doc_to_pengguna(dok)?;

            Ok(Some(peg))
        }
        None => Ok(None)
    }
}
