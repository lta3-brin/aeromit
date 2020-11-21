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

pub fn doc_to_kegiatan(dok: Document) -> Result<Kegiatan, String> {
    let id = dok.get_object_id("_id")
        .map_err(|err| err.to_string())
        .unwrap();

    let kapan = dok.get_datetime("kapan")
        .map_err(|err| err.to_string())
        .unwrap();

    let nama = dok.get_str("nama")
        .map_err(|err| err.to_string())
        .unwrap();

    let ruang = dok.get_str("ruang")
        .map_err(|err| err.to_string())
        .unwrap();

    Ok(Kegiatan {
        id: id.to_hex(),
        nama: nama.to_string(),
        kapan: *kapan,
        ruang: ruang.to_string()
    })
}