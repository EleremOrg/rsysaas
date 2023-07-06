use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize)]

pub struct Term {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub category: String,
    pub tags: String,
}
