//! # Module Handlers
//!
//! Module ini digunakan sebagai _Controller_ atau penggerak atau pengelola berupa
//! fungsi-fungsi _request_/_response_ server.
//!
//! Fungsi-fungsi dari module handlers ini biasa digunakan didalam module `routes`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::{...}
//! ```
pub mod get_user;
pub mod get_users;
pub mod create_user;
pub mod update_user;
pub mod delete_user;
pub mod login_user;
