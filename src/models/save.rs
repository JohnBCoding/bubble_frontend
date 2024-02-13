use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Save {
    pub user_id: String,
    pub article: Article,
}

impl Save {
    pub fn new(user_id: &str, article: &Article) -> Self {
        Self {
            user_id: user_id.to_string(),
            article: article.clone(),
        }
    }
}
