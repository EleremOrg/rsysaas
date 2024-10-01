use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Customer {
    id: u64,
    name: String,
}
