use clap::{Arg, ArgAction, ArgMatches, Command};

use super::migrate;

pub async fn run_cli() {
    let matches = Command::new("AromaCLIck")
        .version("0.1.0")
        .author("Lucas Montes <lluc23@hotmail.com>")
        .about("A CLI for handling migrations and ORM operations")
        .arg(
            Arg::new("migrate")
                .short('m')
                .long("migrate")
                .value_name("Migrate")
                .action(ArgAction::SetTrue)
                .help("Run all the migrations"),
        )
        .get_matches();
    handle_cli(matches).await;
}

async fn handle_cli(matches: ArgMatches) {
    if *matches.get_one("migrate").unwrap_or(&false) {
        migrate("migrations/sqlite").await;
    }
}
