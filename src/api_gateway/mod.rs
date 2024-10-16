mod api_docs;
mod integration;
mod recommendation;
mod routes;
mod shopify;

use routes::routes;
use stefn::Service;

pub fn create_service() -> Service {
    Service::new("api_gateway.json", routes)
}
