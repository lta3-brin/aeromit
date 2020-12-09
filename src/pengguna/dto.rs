//! # Module DTO
//!
//! Module ini digunakan sebagai _Data Transfer Object_ untuk kebutuhan
//! kesamaan masukan/keluaran dari request/response.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::dto::{...}
//! ```
use serde::Deserialize;
use validator::Validate;


/// Struct DocProps diperlukan sebagai pengaturan dokumen.
#[derive(Debug, Deserialize)]
pub struct DocProps {
    /// batas maksimum dokumen yang ditampilkan
    pub limit: Option<i64>,

    /// seberapa banyak dokumen yang dilewati
    pub skip: Option<i64>,

    /// saring pengguna berdasarkan status aktif
    pub isactive: Option<bool>,
}

/// Struct sebagai data transfer object dari pengguna.
#[derive(Debug, Deserialize, Validate)]
pub struct PenggunaDto {
    /// nama pengguna
    #[validate(length(min = 3))]
    pub nama: String,

    /// email pengguna
    #[validate(email)]
    pub email: String,

    /// password pengguna
    #[validate(length(min = 6))]
    pub password: String,

    /// konfirmasi password pengguna
    #[validate(must_match = "password")]
    pub repassword: String,

    /// adminkah pengguna
    pub isadmin: bool,

    /// adminkah pengguna
    pub isactive: bool,
}

/// Struct sebagai data transfer object untuk ubah pengguna.
#[derive(Debug, Deserialize, Validate)]
pub struct UbahPenggunaDto {
    /// nama pengguna
    #[validate(length(min = 3))]
    pub nama: String,

    /// adminkah pengguna
    pub isadmin: bool,

    /// aktifkah pengguna
    pub isactive: bool,
}
