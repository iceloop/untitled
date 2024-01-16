use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: u32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub view_num: Option<u32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct NewArticle {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub content: Option<String>,
    pub view_num: Option<u32>,
    // 不包括 created_at，因为它通常在创建时自动生成并且之后不变
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
