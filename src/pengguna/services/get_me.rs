//! # Module Get Me Service
//!
//! Module ini digunakan untuk membaca data pengguna sendiri berdasarkan token untuk
//! digunakan di `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::get_me::{...}
//! ```
use std::env;
use actix_web::web;
use mongodb::{
    Database,
    bson::{self, doc, document::Document}
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::app::errors::AppErrors;
use crate::pengguna::{
    models::Pengguna,
    helpers::{PenggunaHelpersTrait, PenggunaHelpers},
};
use crate::app::helpers::{AppHelpers, AppHelpersTrait};
use crate::pengguna::models::Klaim;


/// # Fungsi by_token
///
/// Fungsi ini untuk melihat data `Pengguna` sesuai token.
///
/// <br />
///
/// # Masukan
///
/// * `req` - HttpRequest untuk ambil token.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<Option<Pengguna>, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// `Pengguna` dan _Enum_ `AppErrors`.
pub async fn by_token(
    req: web::HttpRequest,
    db: web::Data<Database>
) -> Result<Pengguna, AppErrors> {
    let headers = req.headers().get("authorization");
    let token = <AppHelpers as AppHelpersTrait>::get_token(headers)?;

    if !token.is_empty() {
        let secret = env::var("APP_SECRET")?;

        let klaim = decode::<Klaim>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default()
        )?;

        let email = klaim.claims.get_email();

        let collection = db.collection("pengguna");
        let result = collection
            .find_one(doc! {"email": email}, None)
            .await?;

        match result {
            Some(document) => {
                let dok = bson::from_document::<Document>(document)?;
                let peg = <PenggunaHelpers as PenggunaHelpersTrait>::doc_to_pengguna(dok)?;

                Ok(peg)
            }
            None => Err(AppErrors::UnauthorizeUser)
        }
    } else {
        Err(AppErrors::UnauthorizeUser)
    }
}
