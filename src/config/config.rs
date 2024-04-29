use std::net::{Ipv4Addr};
use crate::server::say_hello_from_server;
use crate::server::server::{Server};
use crate::server::server::Role::_Agg;

#[derive(Debug)]
pub struct Config {
    standalone: bool,
    pub server: Option<Server>,
}


impl Config {
    pub fn new(standalone: bool, server_id: usize, world_size: usize) -> Self {
        let mut conf = Config {
            standalone,
            server: None,
        };
        if standalone {
            conf.server = Option::from(conf.make_local_server(server_id, world_size));
        } else {
            conf.server = Option::from(Server::new(server_id, 1, Ipv4Addr::new(0, 0, 0, 0), world_size));
        }
        conf
    }

    pub fn new_agg(server_id: usize, world_size: usize) -> Self {
        let mut s = Server::new(server_id, 1, Ipv4Addr::new(0, 0, 0, 0), world_size);
        s.set_role(_Agg);
        let mut conf = Config {
            standalone: false,
            server: Some(s),
        };
        conf
    }
    pub fn make_local_server(&self, server_id: usize, worker_size: usize) -> Server {
        let server = Server::new(server_id, worker_size, Ipv4Addr::new(0, 0, 0, 0), 1);
        println!("server{} init, ipv4: {}, port: {}", server_id, server.ipv4_addr, server.port);
        server
    }
    fn make_server() {}
}

pub fn _say_hello() {
    println!("Hello from the config mod!");
    const CALLER: &str = "Config";
    say_hello_from_server(CALLER);
}