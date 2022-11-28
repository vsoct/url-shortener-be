use actix_web::{web::Data, App, HttpServer};
use serde::{Deserialize, Serialize};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use dotenv::dotenv;
use std::env;

mod services;
use services::main::config;

mod database;

struct AppState {
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

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
        .expect("Error building db connection pool.");

    let app_data = Data::new(AppState { db: pool.clone() });

    let address: String = env::var("HOST").expect("HOST environment variable not found.");
    let port: u16 = env::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT environment variable not found.");

    HttpServer::new(move || App::new().app_data(app_data.clone()).configure(config))
        .bind((address, port))?
        .run()
        .await
}
