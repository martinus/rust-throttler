use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str;

use crate::thread_pool::ThreadPool;

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

#[derive(Deserialize, Debug)]
struct Throttling {
    max_parallel: usize,
    priority: i64,
}

#[derive(Deserialize, Debug)]
struct Config {
    num_threads: usize,
    default_priority: i64,
    group: HashMap<String, Throttling>,
}

pub fn run(port: u16, matches: &clap::ArgMatches) {
    let cfg_file = matches.value_of("cfg").unwrap();
    println!("cfg_file={:?}", cfg_file);

    let mut s = String::new();
    File::open(cfg_file)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let cfg: Config = toml::from_str(&s).unwrap();
    println!("cfg={:?}", cfg);

    // locate configuration from ~/.config/
    println!("running server on port {}: {:?}", port, matches);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).unwrap();

    // create a thread pool with the number of elements we have in "default" section
    let pool = ThreadPool::new(cfg.num_threads);
    println!("num_threads={:?}", cfg.num_threads);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_client(stream);
        });
    }
}
