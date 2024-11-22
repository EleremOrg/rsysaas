use stefn::AppError;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::entities::products::Category;

use stefn::shutdown_signal;

use recommender::recommender_client::RecommenderClient;
use recommender::recommender_server::{Recommender, RecommenderServer};
use recommender::{Query, Recommendations};

use super::Recommendation;

mod recommender {
    tonic::include_proto!("recommender");
}

#[derive(Default)]
pub struct RecommenderProxy;

pub async fn serve() -> Result<(), std::io::Error> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<RecommenderServer<RecommenderProxy>>()
        .await;

    let addr = "[::1]:50051".parse().unwrap();
    let proxy = RecommenderProxy::default();

    Server::builder()
        .add_service(health_service)
        .add_service(RecommenderServer::new(proxy))
        .serve_with_shutdown(addr, shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

#[tonic::async_trait]
impl Recommender for RecommenderProxy {
    async fn get_recommendations(
        &self,
        request: Request<Query>,
    ) -> Result<Response<Recommendations>, Status> {
        let reply = Recommendations::default();
        Ok(Response::new(reply))
    }
}

pub struct RecommendationClient(Query);

impl RecommendationClient {
    pub fn new(_category: Category, user_id: i64) -> Self {
        Self(Query {
            version: "1".into(),
            id: "1".into(),
            product_id: None,
            target_id: None,
            user_id,
            quantity: 10,
        })
    }
    pub fn set_target_id(mut self, target_id: String) -> Self {
        self.0.target_id = Some(target_id);
        self
    }

    pub fn set_product_id(mut self, product_id: String) -> Self {
        self.0.product_id = Some(product_id);
        self
    }

    fn track_request(&self) {
        // save that the user has done a request with x params
        // save also the response sent with metadata and track further
    }

    pub async fn recommend(self) -> Result<Vec<Recommendation>, AppError> {
        let mut client = RecommenderClient::connect("http://[::1]:50051")
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?;

        client
            .get_recommendations(tonic::Request::new(self.0))
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))
            .map(|r| {
                r.into_inner()
                    .results
                    .into_iter()
                    .map(|v| Recommendation::new(v.id, v.score))
                    .collect()
            })
    }
}
