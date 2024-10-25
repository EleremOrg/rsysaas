use stefn::Services;

mod core;

pub fn create_service() -> Services {
    Services::new_background_service("server.json", core::run)
}
