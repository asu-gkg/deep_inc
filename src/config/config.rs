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

pub fn make_server() {
    Server::new(0, 1, Ipv4Addr::new(0, 0, 0, 0));
}