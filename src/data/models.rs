//! Generic models to perform calculations
use std::{collections::HashMap, sync::Arc};

use super::RedisManager;
use crate::recsys::{RecRequest, Recommendation};
use rec_rsys::models::{one_hot_encode, Item, ItemAdapter};
use serde::{Deserialize, Serialize};

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
    pub fn get_recommendations(&self, rec_request: RecRequest) -> Vec<Recommendation> {
        match Company::get(rec_request.prod_id) {
            Some(item) => Recommendation::generate_recommendations(
                self.domain.clone(),
                item.to_item(),
                rec_request.num_recs,
            ),
            None => panic!(),
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
    // encode the strings
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
    fn encode_sector(self) -> Vec<f32> {
        let sectors = vec![
            "healthcare",
            "unknow",
            "technology",
            "communication_services",
            "basic_materials",
            "consumer_cyclical",
            "industrials",
            "financial_services",
            "energy",
            "utilities",
            "real_estate",
            "consumer_defensive",
        ];
        match one_hot_encode(&sectors).get(&self.sector) {
            Some(val) => val.to_vec(),
            None => panic!(),
        }
    }
    fn encode_industry(self) -> Vec<f32> {
        let industries = vec![""];
        match one_hot_encode(&industries).get(&self.industry) {
            Some(val) => val.to_vec(),
            None => panic!(),
        }
    }
    fn encode_exchange(self) -> Vec<f32> {
        let exchanges = vec!["NYSE", "NASDAQ", "EURO", "LSE"];
        match one_hot_encode(&exchanges).get(&self.exchange) {
            Some(val) => val.to_vec(),
            None => panic!(),
        }
    }
    fn encode_country(self) -> Vec<f32> {
        let countries = vec!["USA", "FR", "ESP"];
        match one_hot_encode(&countries).get(&self.country) {
            Some(val) => val.to_vec(),
            None => panic!(),
        }
    }
    fn encode_adj(self) -> Vec<f32> {
        let adjs = vec!["growth", "divs", "value", "zombie"];
        Company::sum_encoding_vectors(&one_hot_encode(&adjs), &self.adj)
    }

    fn sum_encoding_vectors(encoding_map: &HashMap<String, Vec<f32>>, adjs: &[String]) -> Vec<f32> {
        let mut sum_vector = vec![0.0; encoding_map.values().next().map_or(0, |v| v.len())];

        for adj in adjs {
            if let Some(encoding) = encoding_map.get(adj) {
                for (sum_value, &enc_value) in sum_vector.iter_mut().zip(encoding.iter()) {
                    *sum_value += enc_value;
                }
            }
        }

        sum_vector
    }
}

impl RedisManager for Company {
    type Item = Company;

    fn prefix() -> String {
        String::from("c")
    }
}

impl ItemAdapter for Company {
    fn to_item(&self) -> Item {
        Item::new(self.id, self.create_values(), None)
    }
    fn create_values(&self) -> Vec<f32> {
        return vec![0.0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_adj() {
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
        assert_eq!(company.encode_adj(), vec![1.0, 1.0, 0.0, 0.0]);
    }
}
