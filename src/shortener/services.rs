use std::{env, sync::MutexGuard};

use super::structs::CreateUrlData;

use super::super::database::models::UrlModel;

use crate::{AppState, Url};

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use nanoid::nanoid;
use serde_json::json;

#[get("/")]
async fn index() -> String {
    "Health check ---- ✔️".to_string()
}

#[post("/")]
async fn create_url(data: web::Data<AppState>, body: web::Json<CreateUrlData>) -> impl Responder {
    let mut shortened_urls = data.shortened_urls.lock().unwrap();
    let now = Utc::now();

    let url_exists = shortened_urls
        .to_vec()
        .into_iter()
        .find(|u| u.url == body.url.clone())
        .is_some();

    if url_exists {
        return HttpResponse::Found().json(json!({
            "short_url": format_shortened_url(shortened_urls, body.url.clone())
        }));
    }

    shortened_urls.push(Url {
        id: nanoid!(10),
        url: body.url.clone(),
        date: now.timestamp(),
    });

    HttpResponse::Created().json(json!({
        "short_url": format_shortened_url(shortened_urls, body.url.clone())
    }))
}

#[get("/{id}")]
async fn expand_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let shortened_urls = data.shortened_urls.lock().unwrap();
    let id = path.into_inner();

    let exist = shortened_urls.to_vec().into_iter().find(|u| u.id == id);

    if exist.is_none() {
        return HttpResponse::NotFound().body("Not found.");
    }

    HttpResponse::Found()
        .append_header(("Location", exist.unwrap().url))
        .finish()
}

#[get("/admin/urls")]
async fn get_urls(data: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().json(data.shortened_urls.lock().unwrap().to_vec())

    match sqlx::query_as::<_, UrlModel>("SELECT id FROM urls")
        .fetch_all(&data.db)
        .await
    {
        Ok(urls) => HttpResponse::Ok().json(urls),
        Err(_) => HttpResponse::NotFound().json("No urls found."),
    }
}

#[delete("/admin/urls/{id}")]
async fn delete_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let mut shortened_urls = data.shortened_urls.lock().unwrap();
    let id = path.into_inner();

    *shortened_urls = shortened_urls
        .to_vec()
        .into_iter()
        .filter(|u| u.id != id)
        .collect();

    HttpResponse::Ok().json(shortened_urls.to_vec())
}

fn format_shortened_url(shortened_urls: MutexGuard<Vec<Url>>, long_url: String) -> String {
    let base_url: String = env::var("BASE_URL").expect("BASE_URL environment variable not found.");

    let id = shortened_urls
        .to_vec()
        .into_iter()
        .find(|u| u.url == long_url)
        .unwrap()
        .id;

    format!("{}/{}", base_url, id)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(expand_url)
        .service(get_urls)
        .service(create_url)
        .service(delete_url);
}
