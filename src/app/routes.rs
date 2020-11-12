use actix_web::web;
use actix_web::web::ServiceConfig;
use crate::kegiatan::routes::kegiatan_routes;
use crate::app::handlers::{app_handler, root_handler};

pub fn root_route(route: &mut ServiceConfig) {
    route
        .service(root_handler)
        .service(
            web::scope("/v1")
                .service(app_handler)
                .configure(kegiatan_routes)
        );
}
