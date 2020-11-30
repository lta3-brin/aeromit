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
use chrono::{DateTime, Utc};


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
    #[validate]
    pub isadmin: bool,

    /// kapan pengguna dibuat
    pub dibuat: DateTime<Utc>,
}
