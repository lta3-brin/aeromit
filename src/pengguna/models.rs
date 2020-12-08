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
pub struct Pengguna {
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

    /// masih aktifkah pengguna
    pub isactive: bool,

    /// kapan pengguna dibuat
    pub dibuat: DateTime<Utc>,

    /// kapan pengguna diubah
    #[serde(rename = "lastModified")]
    pub last_modified: Option<DateTime<Utc>>,
}
