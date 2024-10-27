use axum::Router;
use stefn::AppState;
use tower_http::services::ServeDir;

mod auth;
mod dashboard;
mod seo;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest_service("/dist", ServeDir::new("dist"))
        .merge(auth::routes(state.clone()))
        .merge(dashboard::routes(state.clone()))
        .with_state(state)
}
