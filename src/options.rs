use clap::{App, Arg, ArgMatches, SubCommand};

pub fn get_options_from_cli() -> ArgMatches<'static> {
    App::new("Timekeeper: Sychronize time between two machines.")
        .version("0.1.0")
        .author("Rui Lopes")
        .about("Tool to sychronize time between a master machine and a slave.")
        .subcommand(
            SubCommand::with_name("serve")
                .arg(
                    Arg::with_name("machine-port")
                        .short("p")
                        .long("port")
                        .value_name("server-port")
                        .takes_value(true)
                        .required(false)
                        .default_value("5200")
                        .help("Port where to serve timekeeper's service (if left empty, default value of 5200 will be applied)."),
                )
                .about("Serve time to a local machine on a given port"),
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
                        .help("Timekeeper server machine port where the service is awaiting (if left empty, default value of 5200 will be applied).")
                )
                .about("Connect to a local server machine to update clock"),
        )
        .get_matches()
}
