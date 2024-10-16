use axum::Router;
use rsysaas::api_gateway_service;
use stefn::{AppState, Config};

pub fn setup() -> Router<()> {
    let service = api_gateway_service().stub();
    service.router()
}
