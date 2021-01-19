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

    /// instansi/kampus
    pub instansi: String,

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

    /// Adminkah
    admin: bool,

    /// Kapan klaim diterbitkan
    iat: i64,

    /// Kapan klaim berakhir
    exp: i64,
}

impl Klaim {
    /// # Fungsi new
    ///
    /// Fungsi sebagai "_constructor_" untuk nilai awal _Klaim_.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `subject` - Subjek/nama pengguna.
    /// * `email` - email pengguna.
    /// * `iat` - kapan dibuat.
    /// * `exp` - kapan berakhir.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Klaim` - keluaran berupa _Struct_ klaim
    pub fn new(
        subject: String,
        email: String,
        admin: bool,
        iat: i64,
        exp: i64
    ) -> Self {
        Self {
            sub: subject,
            email,
            admin,
            iat,
            exp
        }
    }

    /// # Fungsi get_email
    ///
    /// Fungsi untuk mengambil field email
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `self` - dapat diakses setelah bagian dari _Struct_ Klaim.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `String` - keluaran berupa String
    pub fn get_email(self) -> String {
        self.email
    }

    /// # Fungsi isadmin
    ///
    /// Fungsi untuk mengetahui status admin
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `self` - dapat diakses setelah bagian dari _Struct_ Klaim.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `bool` - status admin berupa boolean
    pub fn isadmin(self) -> bool {
        self.admin
    }
}
