//! # Module Model
//!
//! Module ini digunakan sebagai _Model Database_ bagian pengguna.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::model::{...}
//! ```
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


/// Model pengguna
#[derive(Debug, Serialize, Deserialize)]
pub struct Pengguna {
    #[serde(rename = "_id")]
    pub id: String,

    /// nama pengguna
    pub nama: String,

    /// email pengguna
    pub email: String,

    /// password pengguna
    #[serde(skip_serializing)]
    pub password: String,

    /// adminkah pengguna
    pub isadmin: bool,

    /// masih aktifkah pengguna
    pub isactive: bool,

    /// kapan pengguna dibuat
    pub dibuat: DateTime<Utc>,

    /// kapan pengguna diubah
    #[serde(rename = "lastModified")]
    pub last_modified: Option<DateTime<Utc>>,
}

/// Model klaim untuk jwt
#[derive(Debug, Serialize, Deserialize)]
pub struct Klaim {
    /// Subject atau pengguna
    sub: String,

    /// Email pengguna
    email: String,

    /// Kapan klaim diterbitkan
    iat: DateTime<Utc>,

    /// Kapan klaim berakhir
    exp: DateTime<Utc>,
}

impl Klaim {
    /// Fungsi sebagai "_constructor_" untuk nilai awal _Klaim_.
    pub fn new(
        subject: String,
        email: String,
        iat: DateTime<Utc>,
        exp: DateTime<Utc>
    ) -> Self {
        Self {
            sub: subject,
            email,
            iat,
            exp
        }
    }
}
