//! # Module app Aeromit
//!
//! Module ini digunakan untuk kelola kegiatan aeromit.
//! Semua bagian dari module kegiatan seperti route, handlers dan lainnya
//! digunakan didalam `main.rs`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! // masukkan kedalam main.rs
//! mod kegiatan
//!
//! use crate::kegiatan::{...}
//! ```
//!
mod dto;
mod models;
mod helpers;
mod handlers;
mod services;
pub mod routes;
