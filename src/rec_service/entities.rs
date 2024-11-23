use super::recommender::{Query, Recommendations};

pub struct RecommendationEngine {
    query: Query,
}

impl RecommendationEngine {
    pub fn new(query: Query) -> Self {
        Self { query }
    }

    pub async fn result(self) -> Recommendations {
        Recommendations::default()
    }
}
