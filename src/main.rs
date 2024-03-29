use std::io;
use std::sync::Arc;

#[macro_use]
extern crate juniper;
extern crate diesel;
extern crate dotenv;

use actix_web::{middleware, get, web, App, Error, HttpServer, HttpResponse, Responder};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use futures::future::Future;

mod graphql_schema;

use crate::graphql_schema::{create_schema, Schema};

#[get("/{id}/{name}/")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}! id:{}", info.1, info.0))
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .service(index)
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
