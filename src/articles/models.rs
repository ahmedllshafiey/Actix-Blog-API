use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateArticle {
    pub title: String,
    pub data: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateArticle {
    pub data: String,
}
