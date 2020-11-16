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
    bson::{self, doc, document::Document, Bson},
    Database,
    options::FindOptions
};
use futures::stream::StreamExt;
use actix_web::{Responder, HttpResponse, web, get};
use crate::app::dto::UmpanBalik;
use crate::kegiatan::models::Kegiatan;

/// # Fungsi baca_kegiatan_handler
///
/// Fungsi ini untuk menampilkan _response_ umpan balik semua kegiatan
/// saat mengunjungi _endpoint root_ `v1/kegiatan`.
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
/// * `impl Responder` - keluaran dari fungsi ini _impl Responder_.
#[get("/kegiatan/")]
pub async fn baca_kegiatan_handler(db: web::Data<Database>) -> impl Responder {
    let mut koleksi_kegiatan: Vec<Kegiatan> = vec![];
    let collection = db.collection("kegiatan");
    let options = FindOptions::builder().sort(doc! { "kapan": -1 }).build();
    let mut cursor = collection.find(None, options).await.unwrap();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                println!("{:?}", document);
                let keg: Kegiatan = bson::from_document(document).unwrap();

                println!("{:?}", keg)
            },
            Err(err) => eprintln!("Error: {:?}", err)
        }
    }

    HttpResponse::Ok().json(UmpanBalik::<Vec<Kegiatan>> {
        sukses: true,
        pesan: "Kegiatan berhasil ditampilkan".to_string(),
        hasil: koleksi_kegiatan
    })
}
