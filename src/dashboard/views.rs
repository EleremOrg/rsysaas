use askama_axum::Template;
use axum::{routing::get, Router};
use stefn::AppState;

use super::seo::Meta;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/register", get(register))
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
#[template(path = "auth/login.html")]
pub struct LoginTemplate {
    meta: Meta,
}

async fn login() -> LoginTemplate {
    let meta = Meta::new(
        "import data".into(),
        "elerem mola".into(),
        "recsys,mola".into(),
        "lucas montes".into(),
        "elerem.com".into(),
        "imafge.com".into(),
    );
   LoginTemplate { meta }
}
#[derive(Template)]
#[template(path = "auth/register.html")]
pub struct RegisterTemplate {
    meta: Meta,
}

async fn register() -> RegisterTemplate {
    let meta = Meta::new(
        "import data".into(),
        "elerem mola".into(),
        "recsys,mola".into(),
        "lucas montes".into(),
        "elerem.com".into(),
        "imafge.com".into(),
    );
    RegisterTemplate { meta }
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
