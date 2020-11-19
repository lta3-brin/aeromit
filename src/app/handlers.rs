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
//! use crate::app::handlers::{...}
//! ```
use actix_web::{Responder, get, HttpResponse};
use crate::app::dto::UmpanBalik;

/// # Fungsi root_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik saat mengunjungi
/// _endpoint root_ `/`.
///
/// <br />
///
/// # Masukan
///
/// * `kosong` - tidak memerlukan masukan dari fungsi ini.
///
/// <br />
///
/// # Keluaran
///
/// * `impl Responder` - keluaran dari fungsi ini _impl Responder_.
#[get("/")]
pub async fn root_handler() -> impl Responder {
    HttpResponse::Ok().json(UmpanBalik::<String> {
        sukses: true,
        pesan: "route utama aeromit".to_string(),
        hasil: "Selamat datang di aplikasi Aeromit".to_string()
    })
}

/// # Fungsi app_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik saat mengunjungi
/// _endpoint_ aplikasi untuk v1 (/v1/), v2 (/v2/), dan lainnya jika tersedia.
///
/// <br />
///
/// # Masukan
///
/// * `kosong` - tidak memerlukan masukan dari fungsi ini.
///
/// <br />
///
/// # Keluaran
///
/// * `impl Responder` - keluaran dari fungsi ini _impl Responder_.
#[get("/")]
pub async fn app_handler() -> impl Responder {
    HttpResponse::Ok().json(UmpanBalik::<String> {
        sukses: true,
        pesan: "route untuk v1".to_string(),
        hasil: "/v1".to_string()
    })
}
