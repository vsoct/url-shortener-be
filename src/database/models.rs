use serde::Serialize;
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow)]
pub struct UrlModel {
    pub id: String,
    pub url: String,
    pub date: i64,
}
