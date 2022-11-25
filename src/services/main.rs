use std::env;

use super::super::database::models::UrlModel;
use super::structs::CreateUrlData;

use crate::AppState;

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
    /*
     * Check if the long url already exist in the database,
     * and return the shortened url instead of creating a new one
     */
    if let Ok(shortened_url) =
        sqlx::query_as::<_, UrlModel>("SELECT id, url, date FROM shortened_urls WHERE url = $1")
            .bind(body.url.to_string())
            .fetch_one(&data.db)
            .await
    {
        return HttpResponse::Ok().json(json!({
            "short_url": format_shortened_url(shortened_url.id)
        }));
    }

    let now = Utc::now();

    match sqlx::query_as::<_, UrlModel>(
        "INSERT INTO shortened_urls (id, url, date) VALUES ($1, $2, $3) RETURNING id, url, date",
    )
    .bind(nanoid!(10))
    .bind(body.url.to_string())
    .bind(now.timestamp())
    .fetch_one(&data.db)
    .await
    {
        Ok(shortened_url) => HttpResponse::Created().json(json!({
            "short_url": format_shortened_url(shortened_url.id)
        })),
        Err(_) => HttpResponse::InternalServerError().json("Could not shorten url."),
    }
}

#[get("/{id}")]
async fn expand_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query_as::<_, UrlModel>("SELECT id, url, date FROM shortened_urls WHERE id = $1")
        .bind(id)
        .fetch_one(&data.db)
        .await
    {
        Ok(shortened_url) => HttpResponse::Found()
            .append_header(("Location", shortened_url.url))
            .finish(),
        Err(_) => HttpResponse::NotFound().json("Not found."),
    }
}

#[get("/admin/urls")]
async fn get_urls(data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, UrlModel>("SELECT id, url, date FROM shortened_urls")
        .fetch_all(&data.db)
        .await
    {
        Ok(urls) => HttpResponse::Ok().json(urls),
        Err(_) => HttpResponse::NotFound().json("No shortened urls found."),
    }
}

#[delete("/admin/urls/{id}")]
async fn delete_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let res = sqlx::query("DELETE FROM shortened_urls WHERE id = $1")
        .bind(id)
        .execute(&data.db)
        .await;

    match res {
        Ok(result) => HttpResponse::Ok().json(format!(
            "{:?} shortened url removed.",
            result.rows_affected()
        )),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete."),
    }
}

fn format_shortened_url(url: String) -> String {
    let base_url: String = env::var("BASE_URL").expect("BASE_URL environment variable not found.");

    format!("{}/{}", base_url, url)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(expand_url)
        .service(get_urls)
        .service(create_url)
        .service(delete_url);
}
