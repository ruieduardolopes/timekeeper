#[macro_use]
extern crate arrayref;

#[macro_use]
use slog::{error, info, o, Drain};
use std::net::Ipv4Addr;
use std::str::FromStr;

pub mod adjuster;
pub mod master;
pub mod messages;
pub mod options;
pub mod slave;
pub mod subcommands;
pub mod utils;

fn main() {
    let clioptions = options::get_options_from_cli();

    // Initialize logger.
    let terminal_decorator = slog_term::TermDecorator::new().build();
    let terminal_drain = slog_term::FullFormat::new(terminal_decorator)
        .build()
        .fuse();
    let terminal_drain = slog_async::Async::new(terminal_drain).build().fuse();

    let log = slog::Logger::root(terminal_drain, o!());

    match clioptions.subcommand_name() {
        Some(subcommand) => match subcommand {
            "serve" => {
                let mut main_address = Ipv4Addr::new(0, 0, 0, 0);
                let mut main_port = 0;
                match clioptions
                        .subcommand_matches(subcommand)
                        .unwrap()
                        .value_of("machine-address") {
                        Some(address) => {
                            // FIXME
                            match Ipv4Addr::from_str(address) {
                                Ok(address) => {main_address = address},
                                Err(error) => {error!(log, "[timekeeper] Timekeeper failed to run as master, since the address could not be parsed as IPv4 address. Address read: {}", address);
                                panic!("{}", error)},
                            }
                        },
                        None => {error!(log, "[timekeeper] Timekeeper failed to run as master, since no address has been successfully read")},
                    };
                match clioptions
                    .subcommand_matches(subcommand)
                    .unwrap()
                    .value_of("machine-port") {
                    Some(port) => {
                        match port.parse() {
                            Ok(port) => {main_port = port},
                            Err(error) => {error!(log, "[timekeeper] Timekeeper failed to run as master, since the port could not be parsed as integer. Port read: {}", port);
                                panic!("{}", error)},
                        }
                    },
                    None => {error!(log, "[timekeeper] Timekeeper failed to run as master, since no port has been successfully read")},
                };
                match subcommands::serve::init(main_address, main_port, log.clone()) {
                    Ok(_) => {}
                    Err(error) => {
                        error!(
                            log,
                            "[timekeeper] Timekeeper master failed to run. Reason: {}", error
                        );
                        panic!("{}", error)
                    }
                }
            }
            "update" => {
                let mut main_address = Ipv4Addr::new(0, 0, 0, 0);
                let mut main_port = 0;
                let dry_run = false;
                match clioptions.subcommand_matches(subcommand) {
                    Some(address) => match address.value_of("machine-address") {
                        Some(address) => match Ipv4Addr::from_str(&address) {
                            Ok(address) => main_address = address,
                            Err(error) => {
                                error!(log, "[timekeeper] Timekeeper failed to run as a slave, since the given address is not correct. Address: {}", address);
                                panic!("{}", error)
                            }
                        },
                        None => {
                            error!(log, "[timekeeper] Timekeeper failed to run as a slave, since no address was given");
                            panic!("No option was given as address.")
                        }
                    },
                    None => {
                        error!(log, "[timekeeper] Timekeeper failed to run as a slave, since no address was given");
                        panic!("No subcommand was given as update")
                    }
                };
                match clioptions.subcommand_matches(subcommand) {
                    Some(port) => match port.value_of("machine-port") {
                        Some(port) => match port.parse() {
                            Ok(port) => main_port = port,
                            Err(error) => {
                                error!(log, "[timekeeper] Timekeeper failed to run as a slave, since the given port is not correct. Port: {}", port);
                                panic!("{}", error)
                            }
                        },
                        None => {
                            error!(log, "[timekeeper] Timekeeper failed to run as a slave, since no port was given");
                            panic!("No option was given as port.")
                        }
                    },
                    None => {
                        error!(log, "[timekeeper] Timekeeper failed to run as a slave, since no port was given");
                        panic!("No subcommand was given as update")
                    }
                }
                match clioptions.subcommand_matches(subcommand) {
                    Some(_) => {
                        info!(log, "[timekeeper] Running synchronization in dry-run mode...");
                        dry_run = true;
                    },
                    None => ()
                }
                match subcommands::update::init(main_address, main_port, dry_run, log.clone()) {
                    Ok(_) => {}
                    Err(error) => {
                        error!(
                            log,
                            "[timekeeper] Timekeeper slave failed to run. Reason: {}", error
                        );
                        panic!("{}", error)
                    }
                }
            }
            _ => panic!("The inserted subcommand of {} is not valid.", subcommand),
        },
        None => {
            panic!("No subcommand was inserted. Please choose one subcommand to perform execution.")
        }
    }
}
