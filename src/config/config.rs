use std::net::{Ipv4Addr};
use crate::server;
use crate::server::server::Server;

#[derive(Debug)]
pub struct Config {
    standalone: bool,
    server: Server,
}

const CALLER: &str = "Config";

impl Config {
    pub fn new(standalone: bool, server_id: usize) -> Self {
        Self {
            standalone,
            server: Config::make_local_server(server_id, 1),
        }
    }

    fn make_local_server(server_id: usize, worker_size: usize) -> Server {
        let server = Server::new(server_id, worker_size, Ipv4Addr::new(0, 0, 0, 0));
        server
    }

    fn make_server() {}
}

pub fn say_hello() {
    println!("Hello from the config mod!");
    server::server::say_hello_from_server(CALLER);
}


