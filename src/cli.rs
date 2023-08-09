use std::env;

use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

use aromatic::run_cli as orm_cli;

use crate::business::facade::CustomerFacade;

pub async fn is_cli_requested() -> bool {
    env::args().collect::<Vec<String>>().len() > 1
}

pub async fn run_cli() {
    let matches = Command::new("ReCLIs")
        .version("0.1.0")
        .author("Lucas Montes <lluc23@hotmail.com>")
        .about("A CLI for handling operations related to envs and database")
        .arg(
            Arg::new("migrate")
                .short('m')
                .long("migrate")
                .value_name("Migrate")
                .action(ArgAction::SetTrue)
                .help("Run all the migrations"),
        )
        .arg(
            Arg::new("create-customer")
                .short('c')
                .long("create-customer")
                .action(ArgAction::Append)
                .value_name("Create a new customer")
                .value_parser(value_parser!(String))
                .number_of_values(4)
                .value_names(&["name", "email", "domain", "models"])
                .help("Create a new customer and get the API keys"),
        )
        .get_matches();
    handle_cli(matches).await;
}

async fn handle_cli(matches: ArgMatches) {
    if *matches.get_one("migrate").unwrap_or(&false) {
        orm_cli().await;
    }

    match matches.get_many::<String>("create-customer") {
        Some(args) => {
            let customer_args = args.clone().collect::<Vec<&String>>();
            CustomerFacade::create_customer(
                customer_args[0],
                customer_args[1],
                customer_args[2],
                customer_args[3],
            )
            .await;
        }
        None => {}
    };
}
