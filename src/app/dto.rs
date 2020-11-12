use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct UmpanBalik<T> {
    pub sukses: bool,
    pub pesan: String,
    pub hasil: T,
}
