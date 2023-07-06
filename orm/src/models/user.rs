use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub customer_id: u32,
}
