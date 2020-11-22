mod app;
mod kegiatan;

use actix_web::{HttpServer, App, middleware};
use crate::app::routes::root_route;
use crate::app::errors::AppErrors;
use crate::app::configs::AppConfigs;


#[actix_web::main]
async fn main() -> Result<(), AppErrors> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = AppConfigs::get_host()?;
    let db = AppConfigs::database_connection().await?;
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
