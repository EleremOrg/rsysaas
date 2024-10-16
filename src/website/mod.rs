mod admin;
mod public;
use public::routes;

use stefn::Service;

pub fn create_service() -> Service {
    Service::new("server.json", routes)
}
