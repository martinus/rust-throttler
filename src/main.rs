extern crate clap;
use clap::{App, Arg, SubCommand};

mod client;
mod server;
mod thread_pool;

// arguments:
fn main() {
    let default_config_path = format!("{}/.throttler.toml", std::env::var("HOME").unwrap());
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
                        .default_value(default_config_path.as_str()),
                ),
        )
        .get_matches();

    // TODO how to properly handle parse errors?
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

    match matches.subcommand() {
        ("server", Some(sub)) => {
            server::run(port, sub);
        }
        ("run", Some(sub)) => {
            let ecode = client::run(port, sub);
            std::process::exit(ecode.code().unwrap_or(0));
        }
        _ => {}
    }

    // gets a value for config if supplied by user, or defaults to "default.conf"
    //let config = matches.value_of("config").unwrap();
    //println!("{:?}", matches);

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
