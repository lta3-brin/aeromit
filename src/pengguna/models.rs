//! # Module Model
//!
//! Module ini digunakan sebagai _Model Database_ bagian pengguna.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::model::{...}
//! ```
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize)]
pub struct Kegiatan {
    #[serde(rename = "_id")]
    pub id: String,

    /// nama pengguna
    pub nama: String,

    /// email pengguna
    pub email: String,

    /// password pengguna
    #[serde(skip_serializing)]
    pub password: String,

    /// adminkah pengguna
    pub isadmin: bool,

    /// kapan pengguna dibuat
    pub dibuat: DateTime<Utc>,
}
