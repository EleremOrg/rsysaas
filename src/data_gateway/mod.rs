mod api_docs;
mod routes;

use routes::routes;
use stefn::HttpService;

pub fn create_service() -> HttpService {
    HttpService::new("data_gateway.json", routes)
}
