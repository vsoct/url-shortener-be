use sqlx::{postgres::PgQueryResult, Error, Pool, Postgres};

use super::models::UrlModel;
use nanoid::nanoid;

pub async fn create_url(url: String, pool: &Pool<Postgres>) -> Result<UrlModel, Error> {
    let shortened_url = sqlx::query_as::<_, UrlModel>(
        "INSERT INTO shortened_urls (id, url) VALUES ($1, $2) RETURNING id, url, created_at",
    )
    .bind(nanoid!(10))
    .bind(url)
    .fetch_one(&*pool)
    .await;

    responder(shortened_url)
}

pub async fn find_by_id(id: String, pool: &Pool<Postgres>) -> Result<UrlModel, Error> {
    let url = sqlx::query_as::<_, UrlModel>(
        "SELECT id, url, created_at FROM shortened_urls WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&*pool)
    .await;

    responder(url)
}

pub async fn find_by_url(url: String, pool: &Pool<Postgres>) -> Result<UrlModel, Error> {
    let url = sqlx::query_as::<_, UrlModel>(
        "SELECT id, url, created_at FROM shortened_urls WHERE url = $1",
    )
    .bind(url)
    .fetch_one(&*pool)
    .await;

    responder(url)
}

pub async fn list_shortened_urls(pool: &Pool<Postgres>) -> Result<Vec<UrlModel>, Error> {
    let urls = sqlx::query_as::<_, UrlModel>("SELECT id, url, created_at FROM shortened_urls")
        .fetch_all(&*pool)
        .await;

    responder(urls)
}

pub async fn delete_shortened_url(
    id: String,
    pool: &Pool<Postgres>,
) -> Result<PgQueryResult, Error> {
    let res = sqlx::query("DELETE FROM shortened_urls WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await;

    responder(res)
}

fn responder<T>(result: Result<T, Error>) -> Result<T, Error> {
    return match result {
        Ok(result) => Ok(result),
        Err(error) => Err(error),
    };
}
