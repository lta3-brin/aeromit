//! # Module Delete User By Id Service
//!
//! Module ini digunakan untuk membaca pengguna berdasarkan id untuk digunakan di `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::delete_user::{...}
//! ```
use actix_web::web;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId}
};
use crate::app::errors::AppErrors;


/// # Fungsi by_id
///
/// Fungsi ini untuk hapus data `Pengguna` sesuai id.
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
/// * `Result<(), AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// seberapa banyak yang dihapus `i64` dan _Enum_ `AppErrors`.
pub async fn by_id(
    uid: web::Path<String>,
    db: web::Data<Database>
) -> Result<i64, AppErrors> {
    let id = ObjectId::with_string(uid.trim())?;
    let collection = db.collection("pengguna");
    let result = collection
        .delete_one(doc! {"_id": id}, None)
        .await?;

    Ok(result.deleted_count)
}
