//! # Module Helpers
//!
//! Module ini digunakan untuk membantu keperluan _module_ masing-masing `services`
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::helpers::{...}
//! ```
use std::env;
use chrono::Utc;
use actix_web::web;
use mongodb::bson::{Document, doc};
use argon2::{self, Config, ThreadMode, Variant, Version};
use crate::app::{
    errors::AppErrors,
    helpers::{AppHelpers, AppHelpersTrait}
};
use crate::pengguna::{
    models::Pengguna,
    dto::{PenggunaDto, UbahPenggunaDto},
};


/// Trait yang digunakan sebagai kerangka fungsi yang dibutuhkan sebagai helpers
pub trait PenggunaHelpersTrait {
    fn doc_to_pengguna(dok: Document) -> Result<Pengguna, AppErrors>;
    fn create_to_doc(payload: web::Form<PenggunaDto>) -> Result<Document, AppErrors>;
    fn update_to_doc(payload: web::Form<UbahPenggunaDto>) -> Result<Document, AppErrors>;
    fn hash_password(password: String) -> Result<String, AppErrors>;
}

/// Struct untuk helpers bagian pengguna
pub struct PenggunaHelpers;

impl PenggunaHelpersTrait for PenggunaHelpers {
    /// # Fungsi doc_to_pengguna
    ///
    /// Fungsi ini untuk mengubah _Mongo Document_ ke _struct_ `Pengguna`.
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
    /// * `Result<Pengguna, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
    /// `Pengguna` dan _Enum_ `AppErrors`.
    fn doc_to_pengguna(dok: Document) -> Result<Pengguna, AppErrors> {
        let id = dok.get_object_id("_id")?;
        let nama = dok.get_str("nama")?;
        let email = dok.get_str("email")?;
        let password = dok.get_str("password")?;
        let admin = dok.get_bool("isadmin")?;
        let aktif = dok.get_bool("isactive")?;
        let kapan = dok.get_datetime("dibuat")?;

        let diubah = <AppHelpers as AppHelpersTrait>::last_modified(
            dok.get("lastModified")
        );

        Ok(Pengguna {
            id: id.to_hex(),
            nama: nama.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            isadmin: admin,
            isactive: aktif,
            dibuat: *kapan,
            last_modified: diubah,
        })
    }

    /// # Fungsi create_to_doc
    ///
    /// Fungsi ini untuk mengubah _struct_ `PenggunaDto` ke _Mongo Document_.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `payload` - masukan dengan _type Struct_ `Pengguna` berupa _DTO_.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<Document, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
    /// `Document` dan _Enum_ `AppErrors`.
    fn create_to_doc(payload: web::Form<PenggunaDto>) -> Result<Document, AppErrors> {
        let admin = is_it_true(payload.0.isadmin);
        let hash = <PenggunaHelpers as PenggunaHelpersTrait>::hash_password(payload.0.password)?;

        Ok(doc! {
            "nama": payload.0.nama,
            "email": payload.0.email,
            "dibuat": Utc::now(),
            "password": hash,
            "isadmin": admin,
            "isactive": true,
            "dibuat": Utc::now(),
        })
    }

    /// # Fungsi update_to_doc
    ///
    /// Fungsi ini untuk mengubah _struct_ `UbahPenggunaDto` ke _Mongo Document_.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `payload` - masukan dengan _type Struct_ `Pengguna` berupa _DTO_.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<Document, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
    /// `Document` dan _Enum_ `AppErrors`.
    fn update_to_doc(payload: web::Form<UbahPenggunaDto>) -> Result<Document, AppErrors> {
        let admin = is_it_true(payload.0.isadmin);
        let aktif = is_it_true(payload.0.isactive);

        Ok(doc! {
            "$set": {
                "nama": payload.0.nama,
                "isadmin": admin,
                "isaktif": aktif,
            },
            "$currentDate": { "lastModified": true }
        })
    }

    /// # Fungsi hash_password
    ///
    /// Fungsi ini untuk melakukan hash pada password.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `password` - password dengan type String.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<Document, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
    /// `Document` dan _Enum_ `AppErrors`.
    fn hash_password(password: String) -> Result<String, AppErrors> {
        let salt = env::var("APP_SALT")?;
        let config = Config {
            variant: Variant::Argon2i,
            version: Version::Version13,
            mem_cost: 65536,
            time_cost: 3,
            lanes: 4,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 32
        };

        let hash = argon2::hash_encoded(
            password.as_bytes(),
            salt.as_bytes(),
            &config
        )?;

        Ok(hash)
    }
}

/// Fungsi untuk cek admin status
fn is_it_true(status: u8) -> bool {
    if status == 0 {
        false
    } else { true }
}
