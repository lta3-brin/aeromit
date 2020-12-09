//! # Module Model
//!
//! Module ini digunakan sebagai _Model Database_ bagian kegiatan.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::kegiatan::model::{...}
//! ```
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize)]
pub struct Kegiatan {
    #[serde(rename = "_id")]
    pub id: String,

    /// nama kegiatan
    pub nama: String,

    /// kapan kegiatan diadakan
    pub kapan: DateTime<Utc>,

    /// dimana kegiatan diadakan
    pub ruang: String,

    /// kapan kegiatan diubah
    #[serde(rename = "lastModified")]
    pub last_modified: Option<DateTime<Utc>>,
}
