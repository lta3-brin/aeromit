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
    bson::{self, doc, Document}
};
use actix_web::web;
use futures::StreamExt;
use crate::kegiatan::models::Kegiatan;

/// # Fungsi baca_kegiatan_service
///
/// Fungsi ini untuk menampilkan keseluruhan data `Kegiatan`
///
/// <br />
///
/// # Masukan
///
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<Vec<Kegiatan>, Error>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `Kegiatan` dan _Struct_ `mongodb::error::Error`.
pub async fn baca_kegiatan_service(db: web::Data<Database>)
    -> Result<Vec<Kegiatan>, mongodb::error::Error> {
    let collection = db.collection("kegiatan");
    let options = FindOptions::builder().sort(doc! { "kapan": -1 }).build();
    let mut cursor = collection.find(None, options).await.unwrap();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                println!("{:?}", document);
                let keg: Document = bson::from_document(document).unwrap();

                println!("{:?}", keg);
            },
            Err(err) => eprintln!("Error: {:?}", err)
        }
    }

    Ok(vec![])
}