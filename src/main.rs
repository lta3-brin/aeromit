mod app;
mod kegiatan;

use std::env;
use actix_web::{HttpServer, App, middleware};
use mongodb::{Client, options::ClientOptions};
use crate::app::routes::root_route;
use crate::app::errors::AppErrors;


#[actix_web::main]
async fn main() -> Result<(), AppErrors> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = env::var("APP_ADDRESS")?;
    let db_addr = env::var("DATABASE_URL")?;
    let db_name = env::var("DEFAULT_DATABASE_NAME")?;

    let mongo_option = ClientOptions::parse(db_addr.as_str()).await?;
    let mongo = Client::with_options(mongo_option)?;
    let db = mongo.database(db_name.as_str());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .data(db.clone())
            .configure(root_route)
    });

    server.bind(host)?
        .run()
        .await
        .map_err(|e| AppErrors::ActixError(e))
}
