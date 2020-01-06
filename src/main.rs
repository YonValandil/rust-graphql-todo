use std::io;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};

#[get("/{id}/{name}/")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! id:{}", info.1, info.0))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Start http server
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}