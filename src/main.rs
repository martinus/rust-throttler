extern crate clap;

use clap::{App, Arg};

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
    println!(
        "'command' values: {:?}",
        matches
            .values_of("command")
            .map(|args| args.collect::<Vec<_>>())
    );
}
