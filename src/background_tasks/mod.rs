use stefn::Service;

mod core;

pub fn create_service() -> Service {
    Service::background(core::run)
}
