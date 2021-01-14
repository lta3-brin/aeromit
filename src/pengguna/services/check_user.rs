//! # Module Check User Service
//!
//! Module ini digunakan untuk periksa pengguna apakah session masih valid
//! dan digunakan di dalam `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::check_user::{...}
//! ```
use std::env;
use actix_web::HttpRequest;
use jsonwebtoken::{DecodingKey, Validation};
use crate::app::errors::AppErrors;
use crate::app::permissions::Roles;
use crate::app::helpers::{
    AppHelpers,
    AppHelpersTrait
};
use crate::pengguna::models::Klaim;


/// # Fungsi run
///
/// Fungsi ini untuk menjalankan fungsi periksa token pengguna.
///
/// <br />
///
/// # Masukan
///
/// * `req` - Actix Http Request
/// * `mode` - jenis periksa berdasarkan peran
///
/// <br />
///
/// # Keluaran
///
/// * `Result<bool, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `bool` dan _Enum_ `AppErrors`.
pub fn run(req: HttpRequest, mode: Roles) -> Result<bool, AppErrors> {
    let headers = req.headers().get("authorization");
    let token = <AppHelpers as AppHelpersTrait>::get_token(headers)?;

    if token.is_empty() {
        Err(AppErrors::UnauthorizeUser)
    } else {
        let secret = env::var("APP_SECRET")?;

        let payload = jsonwebtoken::decode::<Klaim>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default()
        )?;

        match mode {
            Roles::Authenticated => Ok(true),
            Roles::Admin => {
                let isadmin = payload.claims.isadmin();

                Ok(isadmin)
            },
        }
    }
}
