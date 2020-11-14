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
//! use crate::kegiatan::handlers::{...}
//! ```
use actix_web::{Responder, HttpResponse, get};
use crate::app::dto::UmpanBalik;

/// # Fungsi baca_kegiatan_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik semua kegiatan
/// saat mengunjungi _endpoint root_ `v1/kegiatan`.
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
#[get("/kegiatan/")]
pub async fn baca_kegiatan_handler() -> impl Responder {
    HttpResponse::Ok().json(UmpanBalik::<String> {
        sukses: true,
        pesan: "Baca data-data kegiatan".to_string(),
        hasil: "Contoh data-data kegiatan".to_string()
    })
}
