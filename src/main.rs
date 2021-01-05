mod app;
mod pengguna;
mod kegiatan;

use actix_web::{HttpServer, App};
use crate::app::{
    errors::AppErrors,
    routes::root_route,
    configs::AppConfigs,
    middlewares::Middlewares,
};


#[actix_web::main]
async fn main() -> Result<(), AppErrors> {
    AppConfigs::init_config();

    let host = AppConfigs::get_host()?;
    let db = AppConfigs::database_connection().await?;
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Middlewares::handle_session())
            .wrap(Middlewares::set_cors())
            .wrap(Middlewares::build_logger())
            .wrap(Middlewares::normalize_path())
            .data(db.clone())
            .configure(root_route)
    });

    server.bind(host)?
        .run()
        .await
        .map_err(|e| AppErrors::ActixError(e))
}
