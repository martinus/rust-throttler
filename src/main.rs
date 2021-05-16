extern crate clap;
use clap::{App, Arg, SubCommand};
//use std::process::Command;

mod client;
mod server;

// arguments:
fn main() {
    let matches = App::new("throttler")
        .version("0.1")
        .author("Martin Ankerl <martin.ankerl@gmail.com>")
        .about("Throttles concurrent executions")
        .arg(
            Arg::with_name("port")
                .short("p")
                .value_name("PORT")
                .default_value("8765")
                .help("Port of the throttling server"),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Connects to server and waits until it is scheduled")
                .arg(
                    Arg::with_name("group")
                        .short("g")
                        .long("group")
                        .value_name("NAME")
                        .default_value("NOGROUP")
                        .help("Specify to which group the command belongs to")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("command")
                        .value_name("COMMAND")
                        .required(true)
                        .multiple(true)
                        .last(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("Start throttling server")
                .arg(
                    Arg::with_name("cfg")
                        .short("c")
                        .long("config")
                        .value_name("FILE")
                        .default_value("~/.throttler.toml"),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("server") => {
            server::run();
        }
        Some("run") => {
            client::run();
        }
        _ => {}
    }

    // gets a value for config if supplied by user, or defaults to "default.conf"
    //let config = matches.value_of("config").unwrap();
    println!("{:?}", matches);

    /*
    println!("group: {:?}", matches.value_of("group").unwrap());

    let mut command = matches.values_of("command").unwrap();
    let program = command.next().unwrap();
    let args: Vec<&str> = command.collect();

    // for now, just run the given command.

    // spawn a child process and wait until it has finished
    let mut child = Command::new(program)
        .args(args)
        .spawn()
        .expect("failed to execute child");

    // forward exit code
    let ecode = child.wait().expect("failed to wait on child");
    std::process::exit(ecode.code().unwrap_or(0));

    */
}
