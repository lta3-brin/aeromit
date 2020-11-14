mod app;
mod kegiatan;

use std::env;
use actix_web::{HttpServer, App, middleware};
use mongodb::{Client, options::ClientOptions};
use crate::app::routes::root_route;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = env::var("APP_ADDRESS").expect("Env. APP_ADDRESS diperlukan");
    let db_addr = env::var("DATABASE_URL").expect("Env. DATABASE_URL diperlukan");
    let mongo_option = ClientOptions::parse(db_addr.as_str()).await.unwrap();
    let mongo = Client::with_options(mongo_option).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .data(mongo.clone())
            .configure(root_route)
    });

    server
        .bind(host)?
        .run()
        .await
}
