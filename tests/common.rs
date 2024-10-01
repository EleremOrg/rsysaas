use axum::Router;
use stefn::{AppState, Config};
use webservice::custom_routes;

pub fn setup() -> Router<()> {
    let config = Config::stub();
    let state = AppState::new(&config);
    custom_routes(state.clone()).with_state(state.clone())
}
