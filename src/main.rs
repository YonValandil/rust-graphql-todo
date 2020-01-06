use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};

#[get("/{id}/{name}/")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    //format!("Hello {}! id:{}", info.1, info.0)
    HttpResponse::Ok().body("Hello worl !")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}