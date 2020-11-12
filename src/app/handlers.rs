use actix_web::{Responder, get, HttpResponse};
use crate::app::dto::UmpanBalik;

#[get("/")]
pub async fn app_handler() -> impl Responder {
    HttpResponse::Ok().json(UmpanBalik::<String> {
        sukses: true,
        pesan: "route untuk v1".to_string(),
        hasil: "/v1".to_string()
    })
}
