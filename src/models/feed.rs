use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    pub pagination: Pagination,
    pub data: Vec<Article>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: i32,
    pub offset: i32,
    pub count: i32,
    pub total: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Article {
    pub author: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub source: String,
    pub image: String,
    pub category: String,
    pub published_at: String,
}
