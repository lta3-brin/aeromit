//! # Module Get Users Service
//!
//! Module ini digunakan untuk membaca semua pengguna aplikasi untuk digunakan di `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::get_users::{...}
//! ```
use actix_web::web;
use futures::StreamExt;
use mongodb::{
    Database,
    options::FindOptions,
    bson::{self, doc, document::Document}
};
use crate::app::errors::AppErrors;
use crate::pengguna::{
    dto::DocProps,
    models::Pengguna,
    helpers::{PenggunaHelpersTrait, PenggunaHelpers},
};


/// # Fungsi all
///
/// Fungsi ini untuk melihat keseluruhan data `Pengguna`.
/// Apabila terdapat _query_, dapat digunakan sebagai pemilahan data.
///
/// <br />
///
/// # Masukan
///
/// * `doc_props` - properti dokumen untuk pemilahan data pengguna.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<(), AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// `Vec<Pengguna>` dan _Enum_ `AppErrors`.
pub async fn all(
    doc_props: web::Query<DocProps>,
    db: web::Data<Database>
) -> Result<Vec<Pengguna>, AppErrors> {
    let mut pengguna: Vec<Pengguna> = vec![];
    let collection = db.collection("pengguna");

    let doc_limit: i64;
    if let Some(limit) = doc_props.limit {
        doc_limit = limit;
    } else { doc_limit = 10; }

    let is_active: bool;
    if let Some(aktif) = doc_props.isactive {
        is_active = aktif;
    } else { is_active = true; }

    let options = FindOptions::builder()
        .sort(doc! { "nama": -1 })
        .limit(doc_limit)
        .skip(doc_props.skip)
        .build();

    let mut cursor = collection
        .find(doc! {"isactive": is_active}, options)
        .await?;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let dok = bson::from_document::<Document>(document)?;
                let peg = <PenggunaHelpers as PenggunaHelpersTrait>::doc_to_pengguna(dok)?;

                pengguna.push(peg);
            }
            Err(err) => eprintln!("Error: {:?}", err)
        }
    }

    Ok(pengguna)
}
