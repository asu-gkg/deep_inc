use std::net::{IpAddr, Ipv4Addr};
use crate::server;
use crate::server::server::Server;

#[derive(Debug)]
struct Config {
    standalone: bool,
}

const CALLER: &str = "Config";

impl Config {
    pub fn new(standalone: bool) -> Self {
        Self {
            standalone
        }
    }
}

pub fn say_hello() {
    println!("Hello from the config mod!");
    server::server::say_hello_from_server(CALLER);
}

pub fn make_local_server(server_id: usize, worker_size: usize) -> Server {
    let server = Server::new(server_id, worker_size, Ipv4Addr::new(0, 0, 0, 0));
    server
}

pub fn make_server() {}