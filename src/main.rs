use rsysaas::{api_gateway_service, background_service, website_service};
use stefn::ServicesOrquestrator;

fn main() {
    ServicesOrquestrator::new(4, 4)
        .load_environment_variables()
        .run_migrations()
        .add_service(api_gateway_service())
        .add_service(background_service())
        .add_service(website_service())
        .init_dev_tracing()
        .run();
}
