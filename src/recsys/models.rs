use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    prod_id: i16,
    confidence: f64,
    path: String,
}

impl Recommendation {
    pub fn new(prod_id: i16, confidence: f64, path: String) -> Self {
        Recommendation {
            prod_id,
            confidence,
            path,
        }
    }
    pub fn generate_recommendations(num_recs: i8) -> Vec<Recommendation> {
        (0..num_recs)
            .into_iter()
            .map(|x| Recommendation::new(x as i16, x as f64 * 1.3, format!("my/path/{:?}/", x)))
            .collect()
    }
}
