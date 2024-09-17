use axum::http::HeaderValue;
use serde::{Deserialize, Serialize};
use std::{fs::File, net::IpAddr};

use super::{init_dev_tracing, init_prod_tracing};

#[derive(Debug, Serialize, Deserialize)]
pub enum Env {
    Dev,
    Prod,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub env: Env,
    pub ip: IpAddr,
    pub port: u16,
    pub threads: usize,
    pub max_upload_size: u64,
    pub domain_name: String,
    pub allowed_origins: AllowedOrigins,
    pub ips_database: String,
    pub database_url: String,
    pub session_key: String,
}

impl Config {
    pub fn init_tracing(self) -> Self {
        match self.env {
            Env::Dev => init_dev_tracing(),
            Env::Prod => init_prod_tracing(),
        };
        self
    }

    pub fn from_file(file_name: &str) -> Self {
        serde_json::from_reader(File::open(file_name).expect("where is your config file?"))
            .expect("your config is wrong")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AllowedOrigins(Vec<String>);

impl AllowedOrigins {
    fn default() -> Self {
        Self(vec!["*".to_owned()])
    }

    pub fn to_headers(&self) -> Vec<HeaderValue> {
        self.0
            .iter()
            .filter_map(|s| s.parse::<HeaderValue>().ok())
            .collect()
    }
}
