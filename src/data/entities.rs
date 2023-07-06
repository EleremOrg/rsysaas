use super::RedisManager;
use axum::async_trait;

use rec_rsys::models::{one_hot_encode, sum_encoding_vectors, Item, ItemAdapter};

trait CompanyEncoder {
    fn encode_sector(&self) -> Vec<f32>;
    fn encode_industry(&self) -> Vec<f32>;
    fn encode_exchange(&self) -> Vec<f32>;
    fn encode_country(&self) -> Vec<f32>;
    fn encode_adjs(&self) -> Vec<f32>;
}

impl CompanyEncoder for Company {
    fn encode_sector(&self) -> Vec<f32> {
        let sectors = vec![
            "Healthcare",
            "Unknown",
            "Automotive",
            "Technology",
            "Communication Services",
            "Basic Materials",
            "Consumer Cyclical",
            "Industrials",
            "Financial Services",
            "Energy",
            "Utilities",
            "Real Estate",
            "Consumer Defensive",
        ];
        match one_hot_encode(&sectors).get(&self.sector) {
            Some(val) => val.to_vec(),
            None => panic!(),
        }
    }
    fn encode_industry(&self) -> Vec<f32> {
        let industries: Vec<&str> = vec![
            "Technology",
            "Healthcare",
            "Finance",
            "Energy",
            "Unknown",
            "Retail",
            "Manufacturing",
            "Telecommunications",
            "Automotive",
            "Hospitality",
            "Media",
        ];

        match one_hot_encode(&industries).get(&self.industry) {
            Some(val) => val.to_vec(),
            None => panic!(),
        }
    }
    fn encode_exchange(&self) -> Vec<f32> {
        let exchanges = vec![
            "NYSE",
            "NASDAQ",
            "LSE",
            "FWB",
            "TSE",
            "Euronext",
            "BSE",
            "BM&FBOVESPA",
            "SSE",
            "NSE",
        ];
        match one_hot_encode(&exchanges).get(&self.exchange) {
            Some(val) => val.to_vec(),
            None => panic!(),
        }
    }
    fn encode_country(&self) -> Vec<f32> {
        let countries = vec!["USA", "FR", "ESP"];
        match one_hot_encode(&countries).get(&self.country) {
            Some(val) => val.to_vec(),
            None => panic!(),
        }
    }
    fn encode_adjs(&self) -> Vec<f32> {
        let adjs = vec!["growth", "divs", "value", "zombie"];
        sum_encoding_vectors(&one_hot_encode(&adjs), &self.adj)
    }
}

impl RedisManager for Company {
    type Item = Self;

    fn prefix() -> String {
        String::from("c")
    }

    fn handle_not_found() -> Result<Self::Item, CRUDError> {
        Ok(Company::new(
            11,
            "INTC".to_string(),
            "Technology".to_string(),
            "Technology".to_string(),
            "NASDAQ".to_string(),
            "USA".to_string(),
            vec!["growth".to_string(), "divs".to_string()],
            0.3,
        ))
    }
}

pub struct MyCompany(Company);

impl ItemAdapter for MyCompany {
    fn to_item(&self) -> Item {
        Item::new(self.id, self.create_values(), None)
    }
    fn create_values(&self) -> Vec<f32> {
        let mut values = vec![self.growth];
        [
            self.encode_sector(),
            self.encode_industry(),
            self.encode_exchange(),
            self.encode_country(),
            self.encode_adjs(),
        ]
        .iter()
        .for_each(|f| values.extend(f));
        values
    }

    fn get_references(&self) -> Vec<Item> {
        example_companies().iter().map(|c| c.to_item()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_adjs() {
        let company = Company::new(
            1,
            "INTC".to_string(),
            "tech".to_string(),
            "semis".to_string(),
            "nasdaq".to_string(),
            "USA".to_string(),
            vec!["growth".to_string(), "divs".to_string()],
            0.3,
        );
        assert_eq!(company.encode_adjs(), vec![1.0, 1.0, 0.0, 0.0]);
    }
}
