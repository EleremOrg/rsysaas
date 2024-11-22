use rsysaas::{api_gateway_service, background_service, rec_service, website_service};
use stefn::ServicesOrquestrator;

fn main() {
    ServicesOrquestrator::default()
        .load_environment_variables()
        .set_config_from_env()
        .enable_migrations()
        .add_service(rec_service())
        .add_service(api_gateway_service())
        .add_service(background_service())
        .add_service(website_service())
        .init_dev_tracing()
        .run();
}
