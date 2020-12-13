//! # Module Permission Services
//!
//! Module ini digunakan untuk membantu kelola izin pengguna untuk digunakan di `routes`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::permissions::{...}
//! ```

pub mod is_admin;
pub mod is_authenticated;

/// _Struct_ yang digunakan untuk kelola perizinan melalui impl
pub struct UserPermissions;
