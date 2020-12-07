//! # Module Helpers
//!
//! Module ini digunakan untuk membantu keperluan _module_ `services`
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::kegiatan::helpers::{...}
//! ```
use actix_web::web;
use chrono::{DateTime, Utc};
use mongodb::bson::{Document, Bson, doc};
use crate::kegiatan::dto::KegiatanDto;
use crate::kegiatan::models::Kegiatan;
use crate::app::{
    errors::AppErrors,
    helpers::{AppHelpers, AppHelpersTrait}
};


/// Trait yang digunakan sebagai kerangka fungsi yang dibutuhkan sebagai helpers
pub trait KegiatanHelpersTrait {
    fn doc_to_kegiatan(dok: Document) -> Result<Kegiatan, AppErrors>;
    fn kegiatan_to_doc(
        payload: web::Form<KegiatanDto>,
        update: bool
    ) -> Result<Document, AppErrors>;
}

/// Struct untuk helpers bagian kegiatan
pub struct KegiatanHelpers;

impl KegiatanHelpersTrait for KegiatanHelpers {
    /// # Fungsi doc_to_kegiatan
    ///
    /// Fungsi ini untuk mengubah _Mongo Document_ ke _struct_ `Kegiatan`.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `dok` - masukan dengan _type_ `Document`.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<Kegiatan, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
    /// `Kegiatan` dan _Enum_ `AppErrors`.
    fn doc_to_kegiatan(dok: Document) -> Result<Kegiatan, AppErrors> {
        let id = dok.get_object_id("_id")?;
        let kapan = dok.get_datetime("kapan")?;
        let nama = dok.get_str("nama")?;
        let ruang = dok.get_str("ruang")?;

        let diubah = <AppHelpers as AppHelpersTrait>::last_modified(
            dok.get("lastModified")
        );

        Ok(Kegiatan {
            id: id.to_hex(),
            nama: nama.to_string(),
            kapan: *kapan,
            ruang: ruang.to_string(),
            last_modified: diubah
        })
    }

    /// # Fungsi kegiatan_to_doc
    ///
    /// Fungsi ini untuk mengubah _struct_ `Kegiatan` ke _Mongo Document_.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `payload` - masukan dengan _type Struct_ `Kegiatan` berupa _DTO_.
    /// * `update` - status apakah update atau tidak.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<Document, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
    /// `Document` dan _Enum_ `AppErrors`.
    fn kegiatan_to_doc(
        payload: web::Form<KegiatanDto>,
        update: bool
    ) -> Result<Document, AppErrors> {
        let dok: Document;
        let parse_dt = DateTime::parse_from_rfc3339(
            payload.0.kapan.as_str()
        )?;

        let bson_dt: Bson = Bson::DateTime(parse_dt.with_timezone(&Utc));

        if update {
            dok = doc! {
                "$set": {
                    "nama": payload.0.nama,
                    "kapan": bson_dt,
                    "ruang": payload.0.ruang
                }
            };
        } else {
            dok = doc! {
                "nama": payload.0.nama,
                "kapan": bson_dt,
                "ruang": payload.0.ruang
            };
        }

        Ok(dok)
    }
}
