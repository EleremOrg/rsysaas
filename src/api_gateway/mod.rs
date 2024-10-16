mod api_docs;
mod core;
mod routes;
mod shopify;

use routes::routes;
use stefn::HttpService;

pub fn create_service() -> HttpService {
    HttpService::new("api_gateway.json", routes)
}
