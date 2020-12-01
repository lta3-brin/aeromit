//! # Module Services
//!
//! Module ini digunakan untuk melayani fungsi-fungsi yang terdapat didalam module `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::kegiatan::services::{...}
//! ```
use mongodb::{
    Database,
    options::FindOptions,
    bson::{self, doc, oid::ObjectId, Document},
};
use actix_web::web;
use futures::StreamExt;
use validator::Validate;
use crate::app::errors::AppErrors;
use crate::kegiatan::models::Kegiatan;
use crate::kegiatan::dto::{DocProps, KegiatanDto};
use crate::kegiatan::helpers::{KegiatanHelpers, KegiatanHelpersTrait};


/// # Fungsi tambah_kegiatan_service
///
/// Fungsi ini untuk menambahkan data `Kegiatan` baru.
///
/// <br />
///
/// # Masukan
///
/// * `payload` - Data masukan dari pengguna untuk tambah kegiatan.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<(), AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari ()
/// dan _Enum_ `AppErrors`.
pub async fn tambah_kegiatan_service(
    payload: web::Form<KegiatanDto>,
    db: web::Data<Database>,
) -> Result<(), AppErrors> {
    let collection = db.collection("kegiatan");

    payload.validate()?;
    let dok = <KegiatanHelpers as KegiatanHelpersTrait>::kegiatan_to_doc(payload, false)?;

    collection
        .insert_one(dok, None)
        .await?;

    Ok(())
}

/// # Fungsi baca_kegiatan_service
///
/// Fungsi ini untuk menampilkan keseluruhan data `Kegiatan`
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
/// * `Result<Vec<Kegiatan>, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// kumpulan `Kegiatan` dan _Enum_ `AppErrors`.
pub async fn baca_kegiatan_service(
    doc_props: web::Query<DocProps>,
    db: web::Data<Database>
) -> Result<Vec<Kegiatan>, AppErrors> {
    let mut kegiatan: Vec<Kegiatan> = vec![];
    let collection = db.collection("kegiatan");

    let doc_limit: i64;
    if let Some(limit) = doc_props.limit {
        doc_limit = limit;
    } else { doc_limit = 10; }

    let options = FindOptions::builder()
        .sort(doc! { "kapan": -1 })
        .limit(doc_limit)
        .skip(doc_props.skip)
        .build();

    let mut cursor = collection
        .find(None, options)
        .await?;

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let dok = bson::from_document::<Document>(document)?;
                let keg = <KegiatanHelpers as KegiatanHelpersTrait>::doc_to_kegiatan(dok)?;

                kegiatan.push(keg);
            }
            Err(err) => eprintln!("Error: {:?}", err)
        }
    }

    Ok(kegiatan)
}

/// # Fungsi baca_kegiatan_tertentu_service
///
/// Fungsi ini untuk menampilkan data `Kegiatan` tertentu berdasarkan id
///
/// <br />
///
/// # Masukan
///
/// * `uid` - id unik dokumen untuk dipilih.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<Vec<Kegiatan>, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// `Option<Kegiatan>` dan _Enum_ `AppErrors`.
pub async fn baca_kegiatan_tertentu_service(
    uid: web::Path<String>,
    db: web::Data<Database>
) -> Result<Option<Kegiatan>, AppErrors> {
    let id = ObjectId::with_string(uid.trim())?;
    let collection = db.collection("kegiatan");
    let result = collection
        .find_one(doc! {"_id": id}, None)
        .await?;

    match result {
        Some(document) => {
            let dok = bson::from_document::<Document>(document)?;
            let keg = <KegiatanHelpers as KegiatanHelpersTrait>::doc_to_kegiatan(dok)?;

            Ok(Some(keg))
        }
        None => Ok(None)
    }
}

/// # Fungsi ubah_kegiatan_tertentu_service
///
/// Fungsi ini untuk mengubah data `Kegiatan` tertentu berdasarkan id
///
/// <br />
///
/// # Masukan
///
/// * `uid` - id unik dokumen untuk dipilih.
/// * `payload` - Data masukan dari pengguna untuk ubah kegiatan.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<Vec<Kegiatan>, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// `()` dan _Enum_ `AppErrors`.
pub async fn ubah_kegiatan_tertentu_service(
    uid: web::Path<String>,
    payload: web::Form<KegiatanDto>,
    db: web::Data<Database>
) -> Result<(), AppErrors> {
    let collection = db.collection("kegiatan");
    let id = ObjectId::with_string(uid.as_str())?;

    payload.validate()?;
    let dok = <KegiatanHelpers as KegiatanHelpersTrait>::kegiatan_to_doc(payload, true)?;

    collection
        .update_one(doc! {"_id": id}, dok, None)
        .await?;

    Ok(())
}

/// # Fungsi hapus_kegiatan_tertentu_service
///
/// Fungsi ini untuk hapus data `Kegiatan` tertentu berdasarkan id
///
/// <br />
///
/// # Masukan
///
/// * `uid` - id unik dokumen untuk dipilih.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<Vec<Kegiatan>, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// `()` dan _Enum_ `AppErrors`.
pub async fn hapus_kegiatan_tertentu_service(
    uid: web::Path<String>,
    db: web::Data<Database>
) -> Result<(), AppErrors> {
    let id = ObjectId::with_string(uid.trim())?;
    let collection = db.collection("kegiatan");

    collection
        .delete_one(doc! {"_id": id}, None)
        .await?;

    Ok(())
}
