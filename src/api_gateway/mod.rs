mod api_docs;
mod auth;
mod ingestion;
mod recommendation;
mod routes;
mod shopify;

use routes::routes;
use stefn::Service;

pub fn create_service() -> Service {
    Service::api("API_", routes)
}
