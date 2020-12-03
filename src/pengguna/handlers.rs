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
use mongodb::Database;
use actix_web::{
    web,
    get,
    post,
    HttpResponse,
};
use crate::app::errors::AppErrors;
use crate::app::dto::UmpanBalik;
use crate::pengguna::dto::{PenggunaDto, DocProps};
use crate::pengguna::services::{create_user, get_users, get_user};
use crate::pengguna::models::Pengguna;


/// # Fungsi tambah_pengguna_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil penambahan pengguna baru
/// saat mengunjungi _endpoint root_ `v1/pengguna`.
///
/// <br />
///
/// # Masukan
///
/// * `payload` - Data masukan dari pengguna untuk tambah pengguna.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[post("/pengguna/")]
pub async fn tambah_pengguna_handler(
    payload: web::Form<PenggunaDto>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    create_user::new(payload, db).await?;

    Ok(HttpResponse::Created().json(UmpanBalik::<()> {
        sukses: true,
        pesan: "Pengguna berhasil ditambahkan".to_string(),
        hasil: (),
    }))
}

/// # Fungsi baca_pengguna_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil penambahan pengguna baru
/// saat mengunjungi _endpoint root_ `v1/pengguna`.
///
/// <br />
///
/// # Masukan
///
/// * `payload` - Data masukan dari pengguna untuk tambah pengguna.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[get("/pengguna/")]
pub async fn baca_pengguna_handler(
    doc_props: web::Query<DocProps>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    let seluruh_pengguna = get_users::all(doc_props, db).await?;

    Ok(HttpResponse::Ok().json(UmpanBalik::<Vec<Pengguna>> {
        sukses: true,
        pesan: "Pengguna berhasil ditampilkan".to_string(),
        hasil: seluruh_pengguna,
    }))
}

/// # Fungsi baca_pengguna_tertentu_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil baca pengguna sesuai id
/// saat mengunjungi _endpoint root_ `v1/pengguna`.
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
#[get("/pengguna/{id}/")]
pub async fn baca_pengguna_tertentu_handler(
    id: web::Path<String>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    let pengguna_tertentu = get_user::by_id(id, db).await?;

    Ok(HttpResponse::Ok().json(UmpanBalik::<Option<Pengguna>> {
        sukses: true,
        pesan: "Pengguna berhasil ditampilkan".to_string(),
        hasil: pengguna_tertentu,
    }))
}
