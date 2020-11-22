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
use std::time::Duration;

/// Struct `AppConfigs` sebagai wadah konfigurasi aplikasi.
pub struct AppConfigs;

impl AppConfigs {
    /// Fungsi `database_connection` digunakan untuk membuat koneksi database.
    pub async fn database_connection() -> Result<Database, AppErrors> {
        let db_addr = env::var("DATABASE_URL")?;
        let db_name = env::var("DEFAULT_DATABASE_NAME")?;

        let mut mongo_option = ClientOptions::parse(db_addr.as_str()).await?;
        mongo_option.app_name = Some("aeromit".to_string());
        mongo_option.server_selection_timeout = Some(Duration::new(1, 0));

        let mongo = Client::with_options(mongo_option)?;

        Ok(mongo.database(db_name.as_str()))
    }

    /// Fungsi `get_host` digunakan untuk mendapatkan host.
    pub fn get_host() -> Result<String, AppErrors> {
        Ok(env::var("APP_ADDRESS")?)
    }
}
