mod api_gateway;
mod data;
mod data_gateway;
mod recommendation;
mod utils;
mod website;

pub use api_gateway::create_service as api_gateway_service;
pub use data_gateway::create_service as data_gateway_service;
pub use website::create_service as website_service;
