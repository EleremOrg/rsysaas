use std::ops::Deref;

use askama_axum::Template;
use axum::{
    async_trait,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use sqlx::prelude::FromRow;
use stefn::{AppError, Database, WebsiteState};

use crate::website::public::seo::Meta;

pub fn routes(state: WebsiteState) -> Router<WebsiteState> {
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

#[derive(Template)]
#[template(path = "admin/list.html")]
pub struct AdminListTemplate<T> {
    pub items: Vec<T>,
}

#[async_trait]
pub trait AdminModel<R: sqlx::Row, J, Pk: 'static = i64>:
    for<'a> FromRow<'a, R> + Sized + Send + Sync
where
    Self: 'static + Sized + Send + Sync,
{
    type Create;
    fn routes(&self, state: WebsiteState) -> Router<WebsiteState> {
        let path = self.path();
        Router::new()
            .route("/dashboard", post(Self::post).put(Self::put))
            .route("/import-data", get(import_data))
            .with_state(state)
    }

    async fn post(pool: State<Database>, item: Form<J>) -> impl IntoResponse;
    async fn put(pool: State<Database>) -> impl IntoResponse;
    fn path(&self) -> &str;

    fn get_id(&self) -> Pk;

    async fn create(pool: &Database, item: Self) -> Result<Self, AppError>;
    async fn get_by_id(pool: &Database, id: Pk) -> Result<Option<Self>, AppError>;
    async fn update(pool: &Database, id: Pk, updated_item: Self) -> Result<Self, AppError>;
    async fn delete(pool: &Database, id: Pk) -> Result<i64, AppError>;
    async fn list_all(pool: &Database) -> Result<Vec<Self>, AppError>;
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

#[derive(Template)]
#[template(path = "deep-base.html")]
struct DeepBaseTemplate {
    year: u16,
}

#[derive(Template)]
#[template(path = "deep-mid.html")]
struct DeepMidTemplate {
    _parent: DeepBaseTemplate,
    title: String,
}

#[derive(Template)]
#[template(path = "deep-kid.html")]
struct DeepKidTemplate {
    _parent: DeepMidTemplate,
    item: String,
}

impl Deref for DeepKidTemplate {
    type Target = DeepMidTemplate;

    fn deref(&self) -> &Self::Target {
        &self._parent
    }
}
