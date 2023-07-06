use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize)]

pub struct Association {
    pub id: u32,
    pub table_related: String,
    pub row_id: u32,
}
