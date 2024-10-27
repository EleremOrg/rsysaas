use axum::Router;
use rsysaas::api_gateway_service;
use stefn::Service;

pub async fn setup() -> Router<()> {
    let service = api_gateway_service().stub();
    service.run_migrations().await;
    service.router().unwrap()
}