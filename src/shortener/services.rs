use super::models::CreateUrlData;
use crate::{AppState, Url};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use nanoid::nanoid;

#[get("/")]
async fn index() -> String {
    "Health check ---- ✔️".to_string()
}

#[get("/urls")]
async fn get_urls(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.shortened_urls.lock().unwrap().to_vec())
}

#[post("/urls")]
async fn create_url(data: web::Data<AppState>, body: web::Json<CreateUrlData>) -> impl Responder {
    let mut shortened_urls = data.shortened_urls.lock().unwrap();
    let mut max_id: i32 = 0;
    let now = Utc::now();

    // Very inefficient but will do for now.
    for i in 0..shortened_urls.len() {
        if shortened_urls[i].id > max_id {
            max_id = shortened_urls[i].id;
        }
    }
    shortened_urls.push(Url {
        id: max_id + 1,
        url: body.url.clone(),
        short: format!("https://vs.rl/{}", nanoid!(10)),
        date: now.timestamp(),
    });

    HttpResponse::Ok().json(shortened_urls.to_vec())
}

#[delete("/urls/{id}")]
async fn delete_url(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut shortened_urls = data.shortened_urls.lock().unwrap();
    let id = path.into_inner();

    *shortened_urls = shortened_urls
        .to_vec()
        .into_iter()
        .filter(|u| u.id != id)
        .collect();

    HttpResponse::Ok().json(shortened_urls.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(get_urls)
        .service(create_url)
        .service(delete_url);
}
