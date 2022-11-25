use actix_web::{web::Data, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use dotenv::dotenv;
use std::env;

mod shortener;
use shortener::services;

mod database;

struct AppState {
    shortened_urls: Mutex<Vec<Url>>,
    db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Url {
    id: String,
    url: String,
    date: i64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found.");

    println!("{}", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Error building db connection pool.");

    let app_data = Data::new(AppState {
        shortened_urls: Mutex::new(vec![]),
        db: pool.clone(),
    });

    let address: String =
        env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS environment variable not found.");
    let port: u16 = env::var("SERVER_PORT")
        .unwrap()
        .parse()
        .expect("SERVER_PORT environment variable not found.");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(services::config)
    })
    .bind((address, port))?
    .run()
    .await
}
