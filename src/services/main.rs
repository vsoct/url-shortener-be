use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde_json::json;
use std::env;

use super::super::database::repository;
use super::structs::CreateUrlData;

use crate::AppState;

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

    let already_existing = repository::find_by_url(body.url.to_string(), &data.db).await;

    if let Ok(url_found) = already_existing {
        return HttpResponse::Ok().json(json!({
            "short_url": format_shortened_url(url_found.id)
        }));
    }

    let shortened_url = repository::create_url(body.url.to_string(), &data.db).await;

    match shortened_url {
        Ok(shortened_url) => HttpResponse::Created().json(json!({
            "short_url": format_shortened_url(shortened_url.id)
        })),
        Err(_) => HttpResponse::InternalServerError().json("Could not shorten url."),
    }
}

#[get("/{id}")]
async fn expand_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let found_url = repository::find_by_id(id, &data.db).await;

    match found_url {
        Ok(shortened_url) => HttpResponse::Found()
            .append_header(("Location", shortened_url.url))
            .finish(),
        Err(_) => HttpResponse::NotFound().json("Not found."),
    }
}

#[get("/admin/urls")]
async fn get_urls(data: web::Data<AppState>) -> impl Responder {
    let urls = repository::list_shortened_urls(&data.db).await;

    match urls {
        Ok(urls) => HttpResponse::Ok().json(urls),
        Err(_) => HttpResponse::NotFound().json("No shortened urls found."),
    }
}

#[delete("/admin/urls/{id}")]
async fn delete_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let deleted = repository::delete_shortened_url(id, &data.db).await;

    match deleted {
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
