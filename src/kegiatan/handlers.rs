use actix_web::{Responder, HttpResponse, get};
use crate::app::dto::UmpanBalik;

#[get("/kegiatan/")]
pub async fn baca_kegiatan_handler() -> impl Responder {
    HttpResponse::Ok().json(UmpanBalik::<String> {
        sukses: true,
        pesan: "Baca data-data kegiatan".to_string(),
        hasil: "Contoh data-data kegiatan".to_string()
    })
}
