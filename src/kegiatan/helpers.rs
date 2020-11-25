//! # Module Helpers
//!
//! Module ini digunakan untuk membantu keperluan _module_ `services`
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::kegiatan::services::{...}
//! ```
use mongodb::bson::Document;
use crate::kegiatan::models::Kegiatan;
use crate::app::errors::AppErrors;

pub fn doc_to_kegiatan(dok: Document) -> Result<Kegiatan, AppErrors> {
    let id = dok.get_object_id("_id")?;
    let kapan = dok.get_datetime("kapan")?;
    let nama = dok.get_str("nama")?;
    let ruang = dok.get_str("ruang")?;

    Ok(Kegiatan {
        id: id.to_hex(),
        nama: nama.to_string(),
        kapan: *kapan,
        ruang: ruang.to_string()
    })
}