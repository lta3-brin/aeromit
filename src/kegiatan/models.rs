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


/// Model database kegiatan
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

    /// moderator dalam kegiatan terkait
    pub moderator: String,

    /// pembicara dalam kegiatan terkait
    pub pembicara: Vec<Pembicara>,

    /// adakah tags untuk kegiatan
    pub tags: Option<Vec<String>>,

    /// adakah tautan video kegiatan
    #[serde(rename = "tautanVideo")]
    pub tautan_video: Option<String>,

    /// kapan kegiatan diubah
    #[serde(rename = "lastModified")]
    pub last_modified: Option<DateTime<Utc>>,
}

/// Model Database pembicara
#[derive(Debug, Serialize, Deserialize)]
pub struct Pembicara {
    /// Nama pembicara
    #[serde(default)]
    pub nama: String,

    /// Nama judul presentasi
    #[serde(default)]
    pub judul: String
}
