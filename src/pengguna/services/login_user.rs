//! # Module User Login Service
//!
//! Module ini digunakan untuk membantu proses verifikasi pengguna untuk digunakan di `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::login_user::{...}
//! ```
use argon2;
use std::env;
use actix_web::web;
use validator::Validate;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use mongodb::{
    Database,
    bson::{self, doc, Document},
};
use crate::app::errors::AppErrors;
use crate::pengguna::dto::LoginPenggunaDto;
use crate::pengguna::helpers::{
    PenggunaHelpers,
    PenggunaHelpersTrait
};
use crate::pengguna::models::Klaim;


/// # Fungsi verify
///
/// Fungsi ini untuk verifikasi `Pengguna` sesuai inputan pengguna.
///
/// <br />
///
/// # Masukan
///
/// * `payload` - inputan pengguna berupa email dan password.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<Option<String>, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari
/// `Option<String>` dan _Enum_ `AppErrors`.
pub async fn verify(
    payload: web::Form<LoginPenggunaDto>,
    db: web::Data<Database>
) -> Result<Option<String>, AppErrors> {
    let collection = db.collection("pengguna");

    payload.validate()?;

    let result = collection
        .find_one(doc! {"email": payload.0.email}, None)
        .await?;

    match result {
        Some(document) => {
            let dok = bson::from_document::<Document>(document)?;
            let peg = <PenggunaHelpers as PenggunaHelpersTrait>::doc_to_pengguna(dok)?;
            let hash = peg.password;

            let valid = argon2::verify_encoded(
                &hash,
                payload.0.password.as_bytes()
            )?;

            if valid {
                let secret = env::var("APP_SECRET")?;
                let how_long = env::var("APP_EXPIRE")?;
                let iat = Utc::now();
                let exp = iat + Duration::days(how_long.parse::<i64>()?);
                let klm = Klaim::new(
                    peg.nama,
                    peg.email,
                    peg.isadmin,
                    iat.timestamp_nanos(),
                    exp.timestamp_nanos()
                );

                let token = encode(
                    &Header::default(),
                    &klm,
                    &EncodingKey::from_secret(secret.as_bytes())
                )?;

                Ok(Some(token))
            } else { Ok(None) }
        }

        None => Ok(None)
    }
}
