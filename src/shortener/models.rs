use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateUrlData {
    pub url: String,
}
