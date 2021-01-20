//! # Module services dari pengguna Aeromit
//!
//! Module ini digunakan untuk kelola services dari pengguna aeromit dan didaftarkan
//! kedalam `mod.rs` dari module pengguna.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! // masukkan kedalam mod.rs dari pengguna
//! mod services;
//! ```
pub mod get_users;
pub mod create_user;
pub mod get_user;
pub mod update_user;
pub mod delete_user;
pub mod login_user;
pub mod check_user;
pub mod get_me;
