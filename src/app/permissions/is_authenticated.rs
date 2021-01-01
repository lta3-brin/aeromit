use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::app::errors::AppErrors;
use crate::pengguna::models::Klaim;
use crate::app::permissions::UserPermissions;

impl UserPermissions {
    /// # Fungsi is_authenticated
    ///
    /// Fungsi ini digunakan untuk cek otentikasi pengguna.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `session` - actix session.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<(), AppErrors>` - keluaran berupa enum `Result`
    /// yang terdiri dari () dan _enum_ `AppErrors`
    pub fn is_authenticated() -> Result<(), AppErrors> {
        let error_message = AppErrors::UnauthorizeUser;

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
