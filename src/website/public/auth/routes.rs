use askama_axum::Template;
use axum::{extract::State, response::Redirect, routing::get, Form, Router};
use serde::Deserialize;
use stefn::{hash_password, login_user, AppError, Database, WebsiteState};

use crate::{entities::customers, website::public::seo::Meta};

pub fn routes(state: WebsiteState) -> Router<WebsiteState> {
    Router::new()
        .route("/login", get(login).post(login_user))
        .route("/register", get(register).post(post_register))
        .with_state(state)
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
    company_name: String,
    company_description: String,
    company_domain: String,
    company_country: String,
    company_currency: String,
}

async fn post_register(
    database: State<Database>,
    Form(form): Form<RegisterForm>,
) -> Result<Redirect, AppError> {
    let tx = database
        .get_connection()
        .begin()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;

    let password = hash_password(&form.password)?;

    let tx = customers::Builder::new(tx)
        .user_from_password(&password)
        .await?
        .add_email(&form.email)
        .await?
        .add_to_admin_group()
        .await?
        .customer_from_names(&form.first_name, &form.last_name)
        .await?
        .add_to_new_company(
            &form.company_name,
            &form.company_domain,
            &form.company_description,
            &form.company_country,
            &form.company_currency,
        )
        .await?
        .release();

    tx.commit()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;

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
