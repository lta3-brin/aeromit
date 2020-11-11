use actix_web::{HttpServer, App, Responder, get};


#[get("/")]
async fn app() -> impl Responder {
    format!("Definisi awal project")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new().service(app)
    });

    server
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
