use axum::Router;
use rsysaas::api_gateway_service;
use stefn::Service;

use std::sync::Once;
static INIT: Once = Once::new();

pub fn setup() -> Router<()> {
    INIT.call_once(|| {
        print!("hello from the once");
    });
    print!("hello from the not once");
    let service = api_gateway_service().stub();
    service.router()
}
