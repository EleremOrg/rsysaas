use crate::{
    business::interface::{RecommendationAdapter, RecommendationInterface},
    data::{
        errors::CRUDError,
        interfaces::db::Manager,
        models::invfin::sectors_industries::{Industry, Sector},
    },
    web::interface::View,
};
use aromatic::Orm;
use axum::async_trait;
use futures::stream::StreamExt;
use rec_rsys::models::{one_hot_encode, sum_encoding_vectors, AsyncItemAdapter, Item};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, sqlx::FromRow, Deserialize, Serialize, Default)]

pub struct Company {
    pub id: u32,
    #[sqlx(default)]
    pub ticker: String,
    #[sqlx(default)]
    pub resume: String,
    #[sqlx(default)]
    pub image: String,
    pub sector_id: u32,
    pub industry_id: u32,
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

#[async_trait]
impl AsyncItemAdapter for Company {
    async fn to_item(&self) -> Item {
        Item::new(self.id, self.create_values().await, None)
    }
    async fn create_values(&self) -> Vec<f32> {
        let mut values = vec![self.growth];
        [
            self.encode_sector().await,
            self.encode_industry().await,
            self.encode_exchange(),
            self.encode_country(),
            self.encode_adjs(),
        ]
        .iter()
        .for_each(|f| values.extend(f));
        values
    }
    async fn get_references(&self) -> Vec<Item> {
        match self.get_references_query().await {
            Ok(items) => {
                futures::stream::iter(items)
                    .then(|c| async move { c.to_item().await })
                    .collect::<Vec<Item>>()
                    .await
            }
            Err(_e) => vec![],
        }
    }
}

#[async_trait]
impl RecommendationInterface for Company {
    async fn to_adapter(&self) -> RecommendationAdapter {
        <Company as RecommendationInterface>::new_adapter(
            Company::table().await,
            self.to_item().await,
            self.id,
            self.ticker.clone(),
            self.image.clone(),
            self.resume.clone(),
        )
        .await
    }

    // TODO: take into consideration the fact that a customer may query a table with data from other customers
    async fn get_references_query(&self) -> Result<Vec<Company>, CRUDError> {
        let query = Orm::select(
            "id, ticker, resume, image, sector, industry, exchange, country, adj, growth",
        )
        .from(&Self::table().await)
        .where_clause()
        .not_equal("id", &self.id.to_string())
        .ready();
        Self::rows_to_vec(
            format!("SELECT * FROM {}", Self::table().await),
            Self::transaction().await?,
        )
        .await
    }
}

impl Company {
    async fn encode_sector(&self) -> Vec<f32> {
        let (own_sector, sectors) = match Sector::get_for_encoding(self.sector_id).await {
            Ok(sectors) => sectors,
            Err(_e) => return vec![],
        };
        let sectors: Vec<&str> = sectors.iter().map(|f| f.as_str()).collect();
        match one_hot_encode(&sectors).get(&own_sector) {
            Some(val) => val.to_vec(),
            None => panic!(),
        }
    }
    async fn encode_industry(&self) -> Vec<f32> {
        let (own_sindustry, industries) = match Industry::get_for_encoding(self.industry_id).await {
            Ok(industries) => industries,
            Err(_e) => return vec![],
        };
        let industries: Vec<&str> = industries.iter().map(|f| f.as_str()).collect();
        match one_hot_encode(&industries).get(&own_sindustry) {
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
