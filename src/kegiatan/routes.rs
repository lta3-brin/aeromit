use actix_web::web;
use crate::kegiatan::handlers::baca_kegiatan_handler;

pub fn kegiatan_routes(route: &mut web::ServiceConfig) {
    route
        .service(baca_kegiatan_handler);
}
