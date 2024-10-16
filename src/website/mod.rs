mod admin;
mod public;
use public::routes;

use stefn::HttpService;

pub fn create_service() -> HttpService {
    HttpService::new("server.json", routes)
}
