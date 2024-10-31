use askama_axum::Template;
use axum::{response::Redirect, routing::get, Form, Router};
use serde::Deserialize;
use stefn::{AppError, WebsiteState};

use crate::website::public::seo::Meta;

pub fn routes(state: WebsiteState) -> Router<WebsiteState> {
    Router::new()
        .route("/login", get(login).post(post_login))
        .route("/register", get(register).post(post_register))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

async fn post_login(Form(input): Form<LoginForm>) -> Result<Redirect, AppError> {
    let r = "";
    Ok(Redirect::to(r))
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

#[derive(Deserialize)]
pub struct RegisterForm {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

async fn post_register(Form(input): Form<RegisterForm>) -> Result<Redirect, AppError> {
    let r = "";
    Ok(Redirect::to(r))
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
