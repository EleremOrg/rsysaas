use menva::read_default_file;
use rsysaas::{api_gateway_service, data_gateway_service, website_service};
use stefn::ServicesOrquestrator;

fn main() {
    read_default_file();

    ServicesOrquestrator::new(4, 4)
        .add_service(api_gateway_service())
        .add_service(data_gateway_service())
        .add_service(website_service())
        .run();
}
