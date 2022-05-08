use actix_redis::RedisActor;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use std::env;

mod db;

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
    let conn_db_pool = db::create_db_connection_pool();
    let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let redis_port = env::var("REDIS_HOST").unwrap_or_else(|_| "6379".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(conn_db_pool.clone())
            .app_data(RedisActor::start(format!("{}:{}", redis_host, redis_port)))
            .route("/", web::get().to(index))
            .route("/hey", web::get().to(manual_hello))
            .service(actix_files::Files::new("", "build"))
            .default_service(web::route().to(index))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
