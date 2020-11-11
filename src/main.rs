mod app;

use std::env;
use actix_web::{HttpServer, App, middleware};
use crate::app::routes::AppRoutes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = env::var("APP_ADDRESS").expect("Env. APP_ADDRESS diperlukan");
    let server = HttpServer::new(|| {
        let route = AppRoutes::new();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .service(route.root)
            .service(route.v1)
    });

    server
        .bind(host)?
        .run()
        .await
}
