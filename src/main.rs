use app::custom_routes;
use stefn::run;

fn main() {
    run("server.json", custom_routes)
}
