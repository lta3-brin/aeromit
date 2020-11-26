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
use mongodb::{
    bson::doc,
    Database,
};
use actix_web::{
    get,
    web,
    HttpResponse,
};
use crate::app::dto::UmpanBalik;
use crate::app::errors::AppErrors;
use crate::kegiatan::dto::DocProps;
use crate::kegiatan::models::Kegiatan;
use crate::kegiatan::services::{baca_kegiatan_service, baca_kegiatan_tertentu_service};

/// # Fungsi baca_kegiatan_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik semua kegiatan
/// saat mengunjungi _endpoint root_ `v1/kegiatan`.
///
/// <br />
///
/// # Masukan
///
/// * `doc_props` - properti dokumen untuk kelola limit dan skip.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[get("/kegiatan/")]
pub async fn baca_kegiatan_handler(doc_props: web::Query<DocProps>, db: web::Data<Database>)
                                   -> Result<HttpResponse, AppErrors> {
    let koleksi_kegiatan = baca_kegiatan_service(doc_props, db).await?;

    Ok(HttpResponse::Ok().json(UmpanBalik::<Vec<Kegiatan>> {
        sukses: true,
        pesan: "Kegiatan berhasil ditampilkan".to_string(),
        hasil: koleksi_kegiatan,
    }))
}

/// # Fungsi baca_kegiatan_tertentu_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik kegiatan tertentu
/// berdasarkan id saat mengunjungi _endpoint root_ `v1/kegiatan/{id}`.
///
/// <br />
///
/// # Masukan
///
/// * `id` - id dokumen yang ingin ditelusuri.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[get("/kegiatan/{id}/")]
pub async fn baca_kegiatan_tertentu_handler(id: web::Path<String>, db: web::Data<Database>)
                                            -> Result<HttpResponse, AppErrors> {
    let kegiatan = baca_kegiatan_tertentu_service(id, db).await?;

    Ok(HttpResponse::Ok().json(UmpanBalik::<Option<Kegiatan>> {
        sukses: true,
        pesan: "Kegiatan berhasil ditampilkan".to_string(),
        hasil: kegiatan,
    }))
}
