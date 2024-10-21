use stefn::{Service, ServiceConfig};

mod core;
mod prestashop;

#[derive(Clone)]
pub struct BackgroundService {
    config: ServiceConfig,
}

impl BackgroundService {
    pub fn new(config_path: &str) -> Self {
        Self {
            config: ServiceConfig::from_file(config_path),
        }
    }
}

impl Service for BackgroundService {
    fn stub(self) -> Self {
        Self {
            config: ServiceConfig::stub(),
        }
    }

    async fn run(&self) -> Result<(), std::io::Error> {
        core::run().await
    }
}
pub fn create_service() -> BackgroundService {
    BackgroundService::new("server.json")
}
