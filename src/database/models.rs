use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::Serialize;
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow)]
pub struct UrlModel {
    pub id: String,
    pub url: String,
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
}
