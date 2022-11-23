use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use dotenv::dotenv;
// use std::env;

mod shortener;
use shortener::services;

struct AppState {
    shortened_urls: Mutex<Vec<Url>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Url {
    id: i32,
    url: String,
    short: String,
    date: i64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // println!(
    //     "DATABASE_URL: {:?}",
    //     env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found.")
    // );

    let app_data = web::Data::new(AppState {
        shortened_urls: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
