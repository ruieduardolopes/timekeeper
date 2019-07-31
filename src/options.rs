use clap::{App, Arg, ArgMatches, SubCommand};

pub fn get_options_from_cli() -> ArgMatches<'static> {
    App::new("Timekeeper: Synchronize time between two machines.")
        .version("v1.1.3")
        .author("Rui Lopes")
        .about("Tool to synchronize time between a master machine and a slave.")
        .subcommand(
            SubCommand::with_name("serve")
                .arg(
                    Arg::with_name("machine-address")
                        .short("a")
                        .long("address")
                        .value_name("ipv4-address")
                        .takes_value(true)
                        .required(true)
                        .help("Address where to serve timekeeper's service ."),
                )
                .arg(
                    Arg::with_name("machine-port")
                        .short("p")
                        .long("port")
                        .value_name("server-port")
                        .takes_value(true)
                        .required(false)
                        .default_value("5200")
                        .help("Port where to serve timekeeper's service."),
                )
                .about("Serve time to a local machine on a given address and port"),
        )
        .subcommand(
            SubCommand::with_name("update")
                .arg(
                    Arg::with_name("machine-address")
                        .short("s")
                        .long("server")
                        .value_name("server-address")
                        .takes_value(true)
                        .required(true)
                        .help("Timekeeper server machine address to whom a synchronization is to be done.")
                )
                .arg(
                    Arg::with_name("machine-port")
                        .short("p")
                        .long("port")
                        .value_name("server-port")
                        .takes_value(true)
                        .required(false)
                        .default_value("5200")
                        .help("Timekeeper server machine port where the service is awaiting.")
                )
                .arg(
                    Arg::with_name("dry-run")
                        .short("d")
                        .long("dry-run")
                        .required(false)
                        .help("Execute synchronization without changing clocks")
                )
                .about("Connect to a local server machine to update clock"),
        )
        .get_matches()
}
