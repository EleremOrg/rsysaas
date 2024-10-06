use menva::read_default_file;
use stefn::run;
use webservice::custom_routes;

fn main() {
    read_default_file();
    run("server.json", custom_routes)
}
