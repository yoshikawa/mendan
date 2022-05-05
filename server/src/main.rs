use std::env;

use actix_files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
type RedisConn = r2d2_redis::r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>;

async fn index() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("build/index.html")?)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let psql_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://mendan:password@db/mendan".to_string());
    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let redis_port = env::var("REDIS_HOST").unwrap_or_else(|_| "6379".to_string());

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hey", web::get().to(manual_hello))
            .service(actix_files::Files::new("", "build"))
            .default_service(
                web::route().to(index)
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}