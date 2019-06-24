#[macro_use]
extern crate arrayref;

use std::net::Ipv4Addr;
use std::str::FromStr;

pub mod master;
pub mod messages;
pub mod options;
pub mod slave;
pub mod subcommands;
pub mod time_adjuster;
pub mod utils;

fn main() {
    let clioptions = options::get_options_from_cli();

    match clioptions.subcommand_name() {
        Some(subcommand) => match subcommand {
            "serve" => match subcommands::serve::init(
                clioptions
                    .subcommand_matches(subcommand)
                    .unwrap()
                    .value_of("machine-port")
                    .expect("Bad machine port")
                    .parse()
                    .expect("Bad parsing"),
            ) {
                Ok(_) => {}
                Err(_) => {}
            },
            "update" => match subcommands::update::init(
                Ipv4Addr::from_str(
                    &clioptions
                        .subcommand_matches(subcommand)
                        .unwrap()
                        .value_of("machine-address")
                        .unwrap(),
                )
                .unwrap(),
                clioptions
                    .subcommand_matches(subcommand)
                    .unwrap()
                    .value_of("port")
                    .unwrap()
                    .parse()
                    .unwrap(),
            ) {
                Ok(_) => {}
                Err(_) => {}
            },
            _ => panic!("The inserted subcommand of {} is not valid.", subcommand),
        },
        None => {
            panic!("No subcommand was inserted. Please choose one subcommand to perform execution.")
        }
    }
}
