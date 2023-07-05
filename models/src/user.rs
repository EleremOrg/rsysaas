#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub customer_id: u32,
}
