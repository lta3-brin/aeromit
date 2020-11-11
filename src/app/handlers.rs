use actix_web::Responder;

#[get("/")]
pub async fn app_handler() -> impl Responder {
    format!("Route v1")
}