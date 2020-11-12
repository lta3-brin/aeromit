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
//! use crate::app::dto::{...}
//! ```
use serde::{Serialize};

/// Struct sebagai data transfer object untuk umpan balik.
/// Struct UmpanBalik memiliki _generic type_ yang diperlukan oleh keluaran
/// dari `hasil`.
#[derive(Debug, Serialize)]
pub struct UmpanBalik<T> {
    /// Umpan balik perlu status apakah sukses atau tidak.
    pub sukses: bool,
    /// Umpan balik perlu informasi dalam bentuk pesan
    pub pesan: String,
    /// Umpan balik perlu keluaran dalam bentuk hasil.
    pub hasil: T,
}
