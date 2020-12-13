//! # Module Permission Services
//!
//! Module ini digunakan untuk membantu kelola izin pengguna untuk digunakan di `routes`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::services::permissions::{...}
//! ```
use std::env;
use actix_session::Session;
use jsonwebtoken::{decode, DecodingKey, Validation};
use actix_web::error::ErrorUnauthorized;
use crate::app::errors::AppErrors;
use crate::pengguna::models::Klaim;


/// _Struct_ yang digunakan untuk kelola perizinan melalui impl
pub struct UserPermissions;

impl UserPermissions {
    pub fn is_authenticated(session: Session) -> Result<(), AppErrors> {
        let has_token = session.get::<String>("masuk")?;
        let error_message = AppErrors::ActixWebError(ErrorUnauthorized("Pengguna tidak terotentikasi"));

        if let Some(token) = has_token {
            let secret = env::var("APP_SECRET")?;

            decode::<Klaim>(
                &token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default()
            )?;

            Ok(())
        } else {
            Err(error_message)
        }
    }
}
