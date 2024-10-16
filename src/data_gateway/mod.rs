mod api_docs;
mod routes;

use routes::routes;
use stefn::Service;

pub fn create_service() -> Service {
    Service::new("data_gateway.json", routes)
}
