use std::env;
use actix_web::{HttpServer, App, Responder, middleware, get};


#[get("/")]
async fn app() -> impl Responder {
    format!("Definisi awal project")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let host = env::var("APP_ADDRESS").expect("Env. APP_ADDRESS diperlukan");
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(app)
    });

    server
        .bind(host)?
        .run()
        .await
}
