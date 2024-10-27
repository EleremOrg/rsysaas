use askama_axum::Template;
use axum::{routing::get, Router};
use stefn::AppState;
use tower_http::services::ServeDir;

use crate::website::public::seo::Meta;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(index))
        .route("/import-data", get(import_data))
        .with_state(state)
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    meta: Meta,
}

async fn index() -> IndexTemplate {
    let meta = Meta::new(
        "dashboard title".into(),
        "elerem mola".into(),
        "recsys,mola".into(),
        "lucas montes".into(),
        "elerem.com".into(),
        "imafge.com".into(),
    );
    IndexTemplate { meta }
}

#[derive(Template)]
#[template(path = "import_data.html")]
pub struct BaseTemplate {
    meta: Meta,
}

async fn import_data() -> BaseTemplate {
    let meta = Meta::new(
        "import data".into(),
        "elerem mola".into(),
        "recsys,mola".into(),
        "lucas montes".into(),
        "elerem.com".into(),
        "imafge.com".into(),
    );
    BaseTemplate { meta }
}
