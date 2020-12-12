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
    let res = UmpanBalik::new(
        true,
        "route utama aeromit",
        "Selamat datang di aplikasi Aeromit"
    );

    HttpResponse::Ok().json(res)
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
    let res = UmpanBalik::new(
        true,
        "route untuk v1",
        "/v1"
    );

    HttpResponse::Ok().json(res)
}
