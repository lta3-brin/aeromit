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
use mongodb::bson::Bson;

/// Trait digunakan untuk menerapkan fungsi yang diperlukan oleh masing-masing `Services`.
pub trait AppHelpersTrait {
    fn last_modified(docu: Option<&Bson>) -> Option<DateTime<Utc>>;
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
    /// * `docu` - masukan dengan _type_ `Document`.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Option<DateTime<Utc>>` - keluaran berupa _enum_ `Result` yang terdiri dari
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
}
