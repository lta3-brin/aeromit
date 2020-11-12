mod app;
mod kegiatan;

use std::env;
use actix_web::{HttpServer, App, middleware};
use crate::app::routes::root_route;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = env::var("APP_ADDRESS").expect("Env. APP_ADDRESS diperlukan");
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .configure(root_route)
    });

    server
        .bind(host)?
        .run()
        .await
}
