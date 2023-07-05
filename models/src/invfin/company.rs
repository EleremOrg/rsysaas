#[derive(Clone, Debug, PartialEq)]

pub struct Company {
    pub id: u32,
    pub ticker: String,
    pub sector: String,
    pub industry: String,
    pub exchange: String,
    pub country: String,
    pub adj: Vec<String>,
    pub growth: f32,
}
