//! Generic models to perform calculations
use crate::recsys::{RecRequest, Recommendation};
use serde::{Deserialize, Serialize};

use super::RedisManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    key: String,
    pub domain: String,
}

impl Customer {
    fn new(token: String) -> Self {
        Customer {
            key: token,
            domain: String::from("invfin"),
        }
    }
    pub fn get_recommendations(self, rec_request: RecRequest) -> Vec<Recommendation> {
        // For this example is a user that clicks on an item to see the description
        let _user = User::get(rec_request.user_id.to_string());
        let item = Company::get(rec_request.prod_id);
        Recommendation::get_recommendations(self.domain, item, rec_request.num_recs)
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
        adj: Vec<String>,
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
}

impl RedisManager for Company {
    type Item = Company;

    fn prefix() -> String {
        String::from("c")
    }
}
