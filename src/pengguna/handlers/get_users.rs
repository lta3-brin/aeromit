//! # Module Get Users Handler
//!
//! Module ini digunakan untuk ambil keseluruhan pengguna untuk digunakan di `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::get_users::{...}
//! ```
use mongodb::Database;
use actix_web::{
    web,
    get,
    HttpResponse,
};
use crate::app::dto::UmpanBalik;
use crate::app::errors::AppErrors;
use crate::pengguna::{
    dto::DocProps,
    models::Pengguna,
    services::get_users,
};

/// # Fungsi all
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil penambahan pengguna baru
/// saat mengunjungi _endpoint root_ `v1/pengguna`.
///
/// <br />
///
/// # Masukan
///
/// * `doc_props` - properti dokumen untuk kelola limit dan skip..
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[get("/pengguna/")]
pub async fn all(
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
