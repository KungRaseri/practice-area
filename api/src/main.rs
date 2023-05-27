mod models;
mod schema;

mod config;
mod types;
mod world;


use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;

use types::{ApiResponse};

use std::{env, sync::Mutex, io::Result};

type DbPool = Pool<Postgres>;

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        status: "200".to_string(),
        message: "OK".to_string()
    })
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("Starting server | Listening on http://localhost:8080/");

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Establishing Database connection pool");

    let pool = PgPoolOptions::new()
                        .max_connections(5)
                        .connect(&database_url)
                        .await
                        .expect("Error building connection pool");

    println!("Database connection pool established");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(status)
            .configure(world::services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
