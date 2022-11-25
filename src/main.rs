use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use dotenv::dotenv;
use std::env;

mod shortener;
use shortener::services;

struct AppState {
    shortened_urls: Mutex<Vec<Url>>,
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

    let app_data = web::Data::new(AppState {
        shortened_urls: Mutex::new(vec![]),
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
