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
//! use crate::kegiatan::dto::{...}
//! ```
use serde::Deserialize;


/// Struct sebagai data transfer object untuk _Query URL_.
/// Struct DocProps diperlukan sebagai pengaturan dokumen.
#[derive(Debug, Deserialize)]
pub struct DocProps {
    /// batas maksimum dokumen yang ditampilkan
    pub limit: Option<i64>,

    /// seberapa banyak dokumen yang dilewati
    pub skip: Option<i64>,
}

/// Struct sebagai data transfer object dari pengguna.
#[derive(Debug, Deserialize)]
pub struct KegiatanDto {
    /// nama kegiatan
    pub nama: String,

    /// kapan kegiatan diadakan
    pub kapan: String,

    /// dimana kegiatan diadakan
    pub ruang: String,
}
