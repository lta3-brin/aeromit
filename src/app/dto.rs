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
    sukses: bool,
    /// Umpan balik perlu informasi dalam bentuk pesan
    pesan: &'static str,
    /// Umpan balik perlu keluaran dalam bentuk hasil.
    hasil: T,
}

impl<T> UmpanBalik<T> {
    pub fn new(sukses: bool, pesan: &'static str, hasil: T) -> Self {
        Self { sukses, pesan, hasil }
    }
}
