extern crate clap;

use clap::{App, Arg};
use std::process::Command;

// arguments:
// throttler tagname a b c ...
fn main() {
    let matches = App::new("throttler")
        .version("0.1")
        .author("Martin Ankerl <martin.ankerl@gmail.com>")
        .about("Throttles concurrent executions")
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
        )
        .get_matches();

    println!("group: {:?}", matches.value_of("group").unwrap());

    let mut command = matches.values_of("command").unwrap();
    let program = command.next().unwrap();
    let args: Vec<&str> = command.collect();

    // for now, just run the given command.

    // spawn a child process
    let mut child = Command::new(program)
        .args(args)
        .spawn()
        .expect("failed to execute child");

    let ecode = child.wait().expect("failed to wait on child");
    std::process::exit(ecode.code().unwrap_or(0));
}
