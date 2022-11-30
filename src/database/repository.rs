use actix_web::Error;
use sqlx::{Pool, Postgres};

use super::models::UrlModel;

// use crate::AppState;

// pub fn create_url() {

// }

// pub fn list_urls() {
//     match sqlx::query_as::<_, UrlModel>("SELECT id FROM urls")
//         .fetch_all(&AppState::db)
//         .await
//     {
//         Ok(urls) => urls,
//         Err(_) =>
//     }
// }

pub async fn list_shortened_urls(pool: &mut Pool<Postgres>) -> Result<Vec<UrlModel>, Error> {
    let urls = sqlx::query_as::<_, UrlModel>("SELECT id, url, created_at FROM shortened_urls")
        .fetch_all(&*pool)
        .await;

    Ok(urls.unwrap())
}
