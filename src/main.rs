use actix_web::{App, HttpServer, Responder, web};
use actix_web::web::Data;
use actix_files::Files;
use dotenv::dotenv;
use sqlx::{postgres::{PgPool, PgPoolOptions}, query_as};
use std::env;

mod requests;
mod models;

async fn greet(request: web::Query<requests::Id>, pool: Data<PgPool>) -> impl Responder {
    let rows: Vec<_> = query_as::<_, models::Category>("
        SELECT id, name, created_at
        FROM categories
        WHERE id > $1
        ORDER BY id DESC
    ")
        .bind(request.id)
        .fetch_all(&**pool)
        .await
        .unwrap();

    web::Json(rows)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Cannot connect to DB");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

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
