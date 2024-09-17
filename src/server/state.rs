use super::auth::Keys;
use axum::extract::{FromRequestParts, State};
use sqlx::SqlitePool;
use std::{ops::Deref, sync::Arc};

pub struct App {
    pub primary_database: SqlitePool,
    pub ips_database: maxminddb::Reader<Vec<u8>>,
    pub keys: Keys,
}

impl App {
    pub fn new(config: &super::config::Config) -> App {
        Self {
            keys: Keys::new(config.session_key.as_bytes()),
            primary_database: SqlitePool::connect_lazy(&config.database_url).unwrap(),
            ips_database: maxminddb::Reader::open_readfile(&config.ips_database).unwrap(),
        }
    }
}

#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
