mod dtos;
mod gateways;
mod movies;
mod skate;

use futures::future::BoxFuture;
use stefn::Service;

use gateways::serve;

pub use dtos::Recommendation;
pub use gateways::RecommendationClient;

pub fn create_service() -> Service {
    Service::background("REC_", run)
}

fn run() -> BoxFuture<'static, Result<(), std::io::Error>> {
    Box::pin(serve())
}
