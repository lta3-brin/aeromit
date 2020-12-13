//! # Module app Aeromit
//!
//! Module ini digunakan untuk kelola aplikasi utama aeromit.
//! Semua bagian dari module app seperti route, handlers dan lainnya
//! digunakan didalam `main.rs`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! // masukkan kedalam main.rs
//! mod app
//!
//! use crate::app::{...}
//! ```
//!
pub mod dto;
mod handlers;
pub mod routes;
pub mod errors;
pub mod configs;
pub mod helpers;
pub mod middlewares;
pub mod permissions;
