use stefn::run;
use webservice::custom_routes;

fn main() {
    run("server.json", custom_routes)
}
