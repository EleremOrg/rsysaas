#[derive(Clone, Debug, PartialEq, Eq)]

pub struct Association {
    pub id: u32,
    pub table_related: String,
    pub row_id: u32,
}
