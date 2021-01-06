//! # Module Helpers
//!
//! Module ini digunakan untuk membantu keperluan _module_ masing-masing `services`
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::app::helpers::{...}
//! ```
use chrono::{DateTime, Utc};
use mongodb::bson::{self, Bson};
use actix_web::http::HeaderValue;
use crate::app::errors::AppErrors;


/// Trait digunakan untuk menerapkan fungsi yang diperlukan oleh masing-masing `Services`.
pub trait AppHelpersTrait {
    fn last_modified(docu: Option<&Bson>) -> Option<DateTime<Utc>>;
    fn optional_string(docu: Option<&Bson>) -> Option<String>;
    fn optional_vector(docu: Option<&Bson>) -> Result<Option<Vec<String>>, AppErrors>;
    fn get_token(headers: Option<&HeaderValue>) -> Result<String, AppErrors>;
}

/// Struct untuk memberikan fungsi-fungsi bantuan melalui implementasi
pub struct AppHelpers;

impl AppHelpersTrait for AppHelpers {
    /// # Fungsi last_modified
    ///
    /// Fungsi ini untuk mendapatkan waktu saat dokumen berhasil diubah.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `docu` - masukan dengan _type_ `Option<&Bson>`.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Option<DateTime<Utc>>` - keluaran berupa _enum_ `Option` yang terdiri dari
    /// `DateTime<Utc>`.
    fn last_modified(docu: Option<&Bson>) -> Option<DateTime<Utc>> {
        let diubah: Option<DateTime<Utc>>;

        if let Some(data) = docu {
            if let Some(kapan) = data.as_datetime() {
                diubah = Some(*kapan)
            } else { diubah = None }
        } else { diubah = None }

        diubah
    }

    /// # Fungsi optional_string
    ///
    /// Fungsi ini untuk mendapatkan optional text saat dokumen berhasil diubah.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `docu` - masukan dengan _type_ `Option<&Bson>`.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Option<String>` - keluaran berupa _enum_ `Option` yang terdiri dari
    /// `String`.
    fn optional_string(docu: Option<&Bson>) -> Option<String> {
        let text: Option<String>;

        if let Some(data) = docu {
            if let Some(kapan) = data.as_str() {
                text = Some(kapan.to_string())
            } else { text = None }
        } else { text = None }

        text
    }

    /// # Fungsi optional_vector
    ///
    /// Fungsi ini untuk mendapatkan optional vector String saat dokumen berhasil diubah.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `docu` - masukan dengan _type_ `Option<&Bson>`.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<Option<Vec<String>>, AppErrors>` - keluaran berupa _enum_ `Option` yang
    /// terdiri dari `Option<Vec<String>` dan AppErrors.
    fn optional_vector(docu: Option<&Bson>) -> Result<Option<Vec<String>>, AppErrors> {
        if let Some(dok) = docu {
            if let Some(tags) = dok.as_array() {
                let mut koleksi: Vec<String> = vec![];
                let tags = tags.to_vec();

                for tag in tags {
                    let tg = bson::from_bson::<String>(tag)?;

                    koleksi.push(tg);
                }

                Ok(Some(koleksi))
            } else {
                Ok(None)
            }
        } else { Ok(None) }
    }

    /// # Fungsi get_token
    ///
    /// Fungsi ini untuk mendapatkan token yang dikirmkan dari pengguna.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `headers` - masukan dengan _type_ `Option<&HeaderValue>`.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<String, AppErrors>` - keluaran berupa _enum_ `Result` yang
    /// terdiri dari `String` dan AppErrors.
    fn get_token(headers: Option<&HeaderValue>) -> Result<String, AppErrors> {
        let token = if let Some(token) = headers {
            let res = token.to_str()?;

             res.replace("Bearer ", "")
        } else {
            "".to_string()
        };

        Ok(token)
    }
}
