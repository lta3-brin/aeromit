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
use actix_web::{post, get, put, delete, web, HttpResponse, HttpRequest};
use crate::app::{
    dto::UmpanBalik,
    errors::AppErrors,
    permissions::UserPermissions
};
use crate::kegiatan::dto::{
    DocProps,
    KegiatanDto,
};
use crate::kegiatan::services::{
    tambah_kegiatan_service,
    baca_kegiatan_service,
    baca_kegiatan_tertentu_service,
    ubah_kegiatan_tertentu_service,
    hapus_kegiatan_tertentu_service,
};


/// # Fungsi tambah_kegiatan_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil penambahan kegiatan baru
/// saat mengunjungi _endpoint root_ `v1/kegiatan`.
///
/// <br />
///
/// # Masukan
///
/// * `payload` - Data masukan dari pengguna untuk tambah kegiatan.
/// * `req` - HttpRequest.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[post("/kegiatan/")]
pub async fn tambah_kegiatan_handler(
    payload: web::Json<KegiatanDto>,
    req: HttpRequest,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(req, db.clone()).await?;

    tambah_kegiatan_service(payload, db).await?;

    let res = UmpanBalik::new(
        true,
        "Kegiatan berhasil ditambahkan",
        ()
    );

    Ok(HttpResponse::Created().json(res))
}

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
/// * `req` - HttpRequest.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[get("/kegiatan/")]
pub async fn baca_kegiatan_handler(
    doc_props: web::Query<DocProps>,
    req: HttpRequest,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_authenticated(req)?;

    let koleksi_kegiatan = baca_kegiatan_service(doc_props, db).await?;

    let res = UmpanBalik::new(
        true,
        "Kegiatan berhasil ditampilkan",
        koleksi_kegiatan
    );

    Ok(HttpResponse::Ok().json(res))
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
/// * `req` - HttpRequest.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[get("/kegiatan/{id}/")]
pub async fn baca_kegiatan_tertentu_handler(
    id: web::Path<String>,
    req: HttpRequest,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_authenticated(req)?;

    let kegiatan = baca_kegiatan_tertentu_service(id, db).await?;

    if kegiatan.is_some() {
        let res = UmpanBalik::new(
            true,
            "Kegiatan berhasil ditampilkan",
            kegiatan
        );

        Ok(HttpResponse::Ok().json(res))
    } else {
        let res = UmpanBalik::new(
            true,
            "Kegiatan tidak ditemukan",
            ()
        );

        Ok(HttpResponse::NotFound().json(res))
    }
}

/// # Fungsi ubah_kegiatan_tertentu_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil ubah kegiatan tertentu
/// berdasarkan id saat mengunjungi _endpoint root_ `v1/kegiatan/{id}`.
///
/// <br />
///
/// # Masukan
///
/// * `id` - id dokumen yang ingin ditelusuri.
/// * `payload` - Data masukan dari pengguna untuk ubah kegiatan.
/// * `req` - HttpRequest.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[put("/kegiatan/{id}/")]
pub async fn ubah_kegiatan_tertentu_handler(
    id: web::Path<String>,
    payload: web::Json<KegiatanDto>,
    req: HttpRequest,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(req, db.clone()).await?;

    let count = ubah_kegiatan_tertentu_service(id, payload, db).await?;

    if count > 0 {
        let res = UmpanBalik::new(
            true,
            "Kegiatan berhasil diubah",
            ()
        );

        Ok(HttpResponse::Ok().json(res))
    } else {
        let res = UmpanBalik::new(
            true,
            "Kegiatan tidak ditemukan",
            ()
        );

        Ok(HttpResponse::NotFound().json(res))
    }
}

/// # Fungsi hapus_kegiatan_tertentu_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil hapus kegiatan tertentu
/// berdasarkan id saat mengunjungi _endpoint root_ `v1/kegiatan/{id}`.
///
/// <br />
///
/// # Masukan
///
/// * `id` - id dokumen yang ingin ditelusuri.
/// * `req` - actix session.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[delete("/kegiatan/{id}/")]
pub async fn hapus_kegiatan_tertentu_handler(
    id: web::Path<String>,
    req: HttpRequest,
    permanent: web::Query<DocProps>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(req, db.clone()).await?;

    let selamanyakah = if let Some(selamanya) = permanent.forever {
        selamanya
    } else {
        false
    };

    let count = hapus_kegiatan_tertentu_service(
        id,
        selamanyakah,
        db
    ).await?;

    if count == 0 {
        let res = UmpanBalik::new(
            true,
            "Kegiatan tidak ditemukan",
            ()
        );

        Ok(HttpResponse::NotFound().json(res))
    } else {
        let res = UmpanBalik::new(
            true,
            "Kegiatan berhasil dihapus",
            ()
        );

        Ok(HttpResponse::Ok().json(res))
    }
}
