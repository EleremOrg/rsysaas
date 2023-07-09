use crate::data::errors::CRUDError;
use crate::data::{facades::db::Manager, orm::Orm};
use crate::web::facade::View;
use axum::async_trait;
use rec_rsys::models::{one_hot_encode, sum_encoding_vectors, Item, ItemAdapter};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

#[derive(Clone, Debug, PartialEq, sqlx::FromRow, Deserialize, Serialize, Default)]

pub struct Company {
    pub id: u32,
    pub ticker: String,
    pub sector: String,
    pub industry: String,
    pub exchange: String,
    pub country: String,
    pub adj: String,
    pub growth: f32,
}

#[async_trait]
impl Manager<'_> for Company {
    async fn table() -> String {
        "companies".to_string()
    }
}
#[async_trait]
impl View<'_> for Company {}

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
    fn get_references(&self) -> Vec<Item> {
        let rt = Runtime::new().unwrap();
        // Call the function asynchronously using the tokio runtime
        let result = rt.block_on(self.get_references_query());

        match result {
            Ok(items) => items.iter().map(|c| c.to_item()).collect(),
            Err(e) => {
                println!("Error: {:?}", e);
                vec![]
            }
        }
    }
}

impl Company {
    async fn get_references_query(&self) -> Result<Vec<Company>, CRUDError> {
        let query = Orm::new()
            .select("id, sector, industry, exchange, country, adj, growth")
            .from(&Self::reg_table())
            .where_clause()
            .not_equal("id", &self.id.to_string())
            .ready();

        let rows = sqlx::query_as::<_, Self>(&query)
            .fetch_all(&mut Self::connect().await)
            .await;

        match rows {
            Ok(json) => Ok(json),
            Err(_e) => Err(CRUDError::WrongParameters),
        }
    }
    fn reg_table() -> String {
        "companies".to_string()
    }

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
        sum_encoding_vectors(
            &one_hot_encode(&adjs),
            &self
                .adj
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        )
    }
}
