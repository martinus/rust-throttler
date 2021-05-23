use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str;
use std::time;

fn handle_client(mut stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    // Wait until we get the group
    let mut data = [0 as u8; 50];
    let size = stream.read(&mut data).unwrap();
    let group = str::from_utf8(&data[0..size]).unwrap();
    println!("got {:?}", group);
    //std::thread::sleep(time::Duration::from_secs(5));

    // send back 1 byte to signal GO
    stream.write_all(b"x").unwrap();

    // waiting until cmd finishes
    stream.read(&mut data).unwrap();
}

pub fn run(port: u16, matches: &clap::ArgMatches) {
    println!("running server on port {}: {:?}", port, matches);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
