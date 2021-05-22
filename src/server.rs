use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{time};

fn handle_client(mut stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());
    let mut data = [0 as u8; 50];
    let size = stream.read(&mut data).unwrap();
    println!("got '{:?}'", &data[0..size]);
    std::thread::sleep(time::Duration::from_secs(5));
    println!("letting go.");
}

pub fn run(port: u16, matches: &clap::ArgMatches) {
    println!("running server on port {}: {:?}", port, matches);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
