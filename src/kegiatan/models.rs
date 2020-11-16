use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kegiatan {
    #[serde(rename="_id")]
    pub id: String,
    pub nama: String,
    pub kapan: DateTime<Utc>,
    pub ruang: String
}