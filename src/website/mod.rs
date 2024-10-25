mod public;
use public::routes;

use stefn::Services;

pub fn create_service() -> Services {
    Services::new_http_service("server.json", routes)
}
