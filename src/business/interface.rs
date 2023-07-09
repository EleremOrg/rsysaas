use crate::{
    business::requests::RecommendationRequest,
    data::{errors::CRUDError, models::customer::Customer},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInterface {}

impl CustomerInterface {
    // fn new(token: Arc<String>) -> Self {
    //     CustomerInterface {
    //         key: token,
    //         domain: "invfin".into(),
    //     }
    // }
    // pub fn get_recommendations(
    //     &self,
    //     rec_request: RecommendationRequest,
    // ) -> Result<Vec<Recommendation>, CRUDError> {
    //     match Entity::get(rec_request.prod_id) {
    //         Ok(item) => Ok(Recommendation::generate_recommendations(
    //             self.domain.clone(),
    //             item.to_item(),
    //             item.get_references(),
    //             rec_request.num_recs,
    //         )),
    //         Err(err) => Err(err),
    //     }
    // }
    pub async fn is_allowed(entity: Arc<String>) -> bool {
        true
    }
}
