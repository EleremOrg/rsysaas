mod api_docs;
mod ingestion;
mod recommendation;
mod routes;
mod shopify;

use routes::routes;
use stefn::Services;

pub fn create_service() -> Services {
    Services::new_http_service("api_gateway.json", routes)
}
