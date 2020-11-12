use actix_web::{Scope, web, HttpResponse, Resource};
use crate::app::handlers::app_handler;

pub struct AppRoutes {
    pub root: Resource,
    pub v1: Scope
}

impl AppRoutes {
    pub fn new() -> Self {
        AppRoutes {
            root: web::resource("/").to(|| HttpResponse::Ok().body("Halaman ini dikosongkan")),
            v1: web::scope("/v1").service(app_handler)
        }
    }
}
