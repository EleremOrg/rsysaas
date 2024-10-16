use axum::Router;
use rsysaas::api_gateway_service;

pub fn setup() -> Router<()> {
    let service = api_gateway_service().stub();
    service.router()
}
