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
use actix_session::Session;
use mongodb::{
    bson::doc,
    Database,
};
use actix_web::{
    post, get,
    put, delete,
    web, HttpResponse,
};
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
/// * `session` - actix session.
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
    payload: web::Form<KegiatanDto>,
    session: Session,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(session, db.clone()).await?;

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
/// * `session` - actix session.
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
    session: Session,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_authenticated(session)?;

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
/// * `session` - actix session.
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
    session: Session,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_authenticated(session)?;

    let kegiatan = baca_kegiatan_tertentu_service(id, db).await?;

    let res = UmpanBalik::new(
        true,
        "Kegiatan berhasil ditampilkan",
        kegiatan
    );

    Ok(HttpResponse::Ok().json(res))
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
/// * `session` - actix session.
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
    payload: web::Form<KegiatanDto>,
    session: Session,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(session, db.clone()).await?;

    ubah_kegiatan_tertentu_service(id, payload, db).await?;

    let res = UmpanBalik::new(
        true,
        "Kegiatan berhasil diubah",
        ()
    );

    Ok(HttpResponse::Ok().json(res))
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
/// * `session` - actix session.
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
    session: Session,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    UserPermissions::is_admin(session, db.clone()).await?;

    hapus_kegiatan_tertentu_service(id, db).await?;

    let res = UmpanBalik::new(
        true,
        "Kegiatan berhasil dihapus",
        ()
    );

    Ok(HttpResponse::Ok().json(res))
}
