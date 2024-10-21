mod api_gateway;
mod data_processing;
mod recommendation;
mod utils;
mod website;

pub use api_gateway::create_service as api_gateway_service;
pub use website::create_service as website_service;
