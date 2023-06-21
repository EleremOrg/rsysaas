//! Generic models to perform calculations
use super::RedisManager;
use crate::data::CRUDError;
use crate::recsys::{RecRequest, Recommendation};

use rec_rsys::models::{one_hot_encode, sum_encoding_vectors, Item, ItemAdapter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub key: String,
    pub domain: Arc<str>,
}

impl Customer {
    fn new(token: String) -> Self {
        Customer {
            key: token,
            domain: "invfin".into(),
        }
    }
    pub fn get_recommendations(
        &self,
        rec_request: RecRequest,
    ) -> Result<Vec<Recommendation>, CRUDError> {
        match Company::get(rec_request.prod_id) {
            Ok(item) => Ok(Recommendation::generate_recommendations(
                self.domain.clone(),
                item.to_item(),
                rec_request.num_recs,
            )),
            Err(err) => Err(err),
        }
    }
    pub fn get(token: String) -> Option<Self> {
        if token == "cool" {
            return Some(Customer::new(token));
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub items: Vec<Company>,
}

impl User {
    pub fn new(id: u32) -> Self {
        User { id, items: vec![] }
    }
    pub fn get(id: String) -> Option<Self> {
        return Some(User::new(id.parse::<u32>().unwrap()));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    id: u32,
    ticker: String,
    sector: String,
    industry: String,
    exchange: String,
    country: String,
    adj: Vec<String>,
    growth: f32,
}

impl Company {
    pub fn new(
        id: u32,
        ticker: String,
        sector: String,
        industry: String,
        exchange: String,
        country: String,
        adj: Vec<String>, // list of adjectives like growth, zombie, divs, value, etc...
        growth: f32,
    ) -> Self {
        Company {
            id,
            ticker,
            sector,
            industry,
            exchange,
            country,
            adj,
            growth,
        }
    }
    fn encode_sector(&self) -> Vec<f32> {
        let sectors = vec![
            "Healthcare",
            "Unknown",
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
    type Item = Company;

    fn prefix() -> String {
        String::from("c")
    }

    fn handle_not_found() -> Result<Self::Item, CRUDError> {
        Ok(Company::new(
            1,
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

impl ItemAdapter for Company {
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
