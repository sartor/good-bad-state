use actix_web::{App, HttpServer, Responder, web};
use actix_web::web::Data;
use actix_files::Files;
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{Database, database::HasStatement, Execute, Executor, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::env;

async fn greet() -> impl Responder {
    format!("OK2")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.expect("Cannot connect to DB");

    HttpServer::new(move || {
        App::new()
            .route("/api/hello", web::get().to(greet))
            .service(Files::new("/", "dist").index_file("index.html").use_last_modified(true))
            .data(pool.clone())
    })
        .bind(env::var("BIND_ADDR").expect("BIND_ADDR must be set"))?
        .run()
        .await
}
