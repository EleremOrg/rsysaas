#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub domain: String,
    pub api_key: String,
}
