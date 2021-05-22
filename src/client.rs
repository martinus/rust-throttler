use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::process::Command;

pub fn run(port: u16, matches: &clap::ArgMatches) -> std::process::ExitStatus {
    let group = matches.value_of("group").unwrap();

    // Connect to server
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let mut stream = TcpStream::connect(addr).unwrap();
    println!("Connected!");
    stream.write_all(group.as_bytes()).unwrap();
    println!("Sent '{}', waiting until connection breaks", group);

    let mut command = matches.values_of("command").unwrap();
    let program = command.next().unwrap();
    let args: Vec<&str> = command.collect();

    // for now, just run the given command.
    let mut data = [0 as u8; 50];
    stream.read(&mut data).unwrap();

    // spawn a child process and wait until it has finished
    let mut child = Command::new(program)
        .args(&args)
        .spawn()
        .expect("failed to execute child");

    println!("program={:?}, args={:?}", program, &args);
    println!("running client on port {}: {:?}", port, matches);

    // forward exit code
    return child.wait().expect("failed to wait on child");
}
