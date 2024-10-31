mod public;
use public::routes;

use stefn::Service;

pub fn create_service() -> Service {
    Service::website("WEB_", routes)
}
