//! # Module Configs
//!
//! Module ini digunakan untuk memberikan pengaturan seperti database aplikasi.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::app::configs::{...}
//! ```
use std::env;
use mongodb::{
    Client,
    Database,
    options::ClientOptions
};
use crate::app::errors::AppErrors;

/// Struct `AppConfigs` sebagai wadah konfigurasi aplikasi.
pub struct AppConfigs;

impl AppConfigs {
    /// Fungsi `database_connection` digunakan untuk membuat koneksi database.
    pub async fn database_connection() -> Result<Database, AppErrors> {
        let db_addr = env::var("DATABASE_URL")?;
        let db_name = env::var("DEFAULT_DATABASE_NAME")?;

        let mongo_option = ClientOptions::parse(db_addr.as_str()).await?;
        let mongo = Client::with_options(mongo_option)?;

        Ok(mongo.database(db_name.as_str()))
    }

    /// Fungsi `get_host` digunakan untuk mendapatkan host.
    pub fn get_host() -> Result<String, AppErrors> {
        Ok(env::var("APP_ADDRESS")?)
    }
}
