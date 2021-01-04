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
use mongodb::bson::{self, Document, Bson, doc};
use crate::kegiatan::dto::KegiatanDto;
use crate::kegiatan::models::{Kegiatan, Pembicara};
use crate::app::{
    errors::AppErrors,
    helpers::{AppHelpers, AppHelpersTrait}
};


/// Trait yang digunakan sebagai kerangka fungsi yang dibutuhkan sebagai helpers
pub trait KegiatanHelpersTrait {
    fn doc_to_kegiatan(dok: Document) -> Result<Kegiatan, AppErrors>;
    fn kegiatan_to_doc(
        payload: web::Json<KegiatanDto>,
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
        let moderator = dok.get_str("moderator")?;
        let dok_pembicara = dok.get_array("pembicara")?;
        let aktifkah = dok.get_bool("aktifkah")?;

        let mut koleksi_pembicara: Vec<Pembicara> = vec![];
        for p in dok_pembicara.to_vec() {
            let pembicara = bson::from_bson::<Pembicara>(p)?;

            koleksi_pembicara.push(pembicara);
        };

        let diubah = <AppHelpers as AppHelpersTrait>::last_modified(
            dok.get("lastModified")
        );

        let tautan_video = <AppHelpers as AppHelpersTrait>::optional_string(
            dok.get("tautanVideo")
        );

        let tags = <AppHelpers as AppHelpersTrait>::optional_vector(
            dok.get("tags")
        )?;

        Ok(Kegiatan {
            id: id.to_hex(),
            nama: nama.to_string(),
            kapan: *kapan,
            ruang: ruang.to_string(),
            moderator: moderator.to_string(),
            pembicara: koleksi_pembicara,
            aktifkah,
            tags,
            tautan_video,
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
        payload: web::Json<KegiatanDto>,
        update: bool
    ) -> Result<Document, AppErrors> {
        let dok: Document;
        let parse_dt = DateTime::parse_from_rfc3339(
            payload.0.kapan.as_str()
        )?;

        let bson_dt: Bson = Bson::DateTime(parse_dt.with_timezone(&Utc));

        let tautan_video = if let Some(tautan) = payload.0.tautan_video {
            Bson::String(tautan)
        } else { Bson::Null };

        let pembicara = payload.0.pembicara
            .into_iter()
            .map(|setiap| {
                let dok = doc! {"nama": setiap.nama, "judul": setiap.judul};

                Bson::Document(dok)
            })
            .collect::<Vec<_>>();

        let tags = if let Some(tags) = payload.0.tags {
            let koleksi = tags.into_iter()
                .map(|setiap| {
                    Bson::String(setiap)
                })
                .collect::<Vec<_>>();

            koleksi
        } else { vec![] };

        if update {
            dok = doc! {
                "$set": {
                    "nama": payload.0.nama,
                    "kapan": bson_dt,
                    "ruang": payload.0.ruang,
                    "tautanVideo": tautan_video,
                    "moderator": payload.0.moderator,
                    "pembicara": pembicara,
                    "aktifkah": true,
                    "tags": tags
                },
                "$currentDate": { "lastModified": true }
            };
        } else {
            dok = doc! {
                "nama": payload.0.nama,
                "kapan": bson_dt,
                "ruang": payload.0.ruang,
                "tautanVideo": tautan_video,
                "moderator": payload.0.moderator,
                "pembicara": Bson::Array(pembicara),
                "aktifkah": true,
                "tags": tags
            };
        }

        Ok(dok)
    }
}
