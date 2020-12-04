//! # Module Helpers
//!
//! Module ini digunakan untuk membantu keperluan _module_ masing-masing `services`
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::{...}
//! ```
use std::env;
use chrono::Utc;
use actix_web::web;
use mongodb::bson::{Document, doc};
use argon2::{self, Config, ThreadMode, Variant, Version};
use crate::app::errors::AppErrors;
use crate::pengguna::dto::PenggunaDto;
use crate::pengguna::models::Pengguna;


/// Trait yang digunakan sebagai kerangka fungsi yang dibutuhkan sebagai helpers
pub trait PenggunaHelpersTrait {
    fn doc_to_pengguna(dok: Document) -> Result<Pengguna, AppErrors>;
    fn pengguna_to_doc(
        payload: web::Form<PenggunaDto>,
        update: bool
    ) -> Result<Document, AppErrors>;
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
        let kapan = dok.get_datetime("dibuat")?;
        let nama = dok.get_str("nama")?;
        let email = dok.get_str("email")?;
        let password = dok.get_str("password")?;
        let admin = dok.get_bool("isadmin")?;

        Ok(Pengguna {
            id: id.to_hex(),
            nama: nama.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            isadmin: admin,
            dibuat: *kapan
        })
    }

    /// # Fungsi pengguna_to_doc
    ///
    /// Fungsi ini untuk mengubah _struct_ `Pengguna` ke _Mongo Document_.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `payload` - masukan dengan _type Struct_ `Pengguna` berupa _DTO_.
    /// * `update` - status apakah update atau tidak.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<Document, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
    /// `Document` dan _Enum_ `AppErrors`.
    fn pengguna_to_doc(
        payload: web::Form<PenggunaDto>,
        update: bool
    ) -> Result<Document, AppErrors> {
        let dok: Document;
        let admin = is_admin(payload.0.isadmin);

        if update {
            dok = doc! {
                "$set": {
                    "nama": payload.0.nama,
                    "isadmin": admin
                }
            };
        } else {
            dok = doc! {
                "nama": payload.0.nama,
                "dibuat": Utc::now(),
                "email": payload.0.email,
                "password": payload.0.password,
                "isadmin": admin
            };
        }

        Ok(dok)
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
            time_cost: 5,
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
fn is_admin(status: u8) -> bool {
    if status == 0 {
        false
    } else { true }
}
