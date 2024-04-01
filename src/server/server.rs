use std::net::{Ipv4Addr};
use crate::server::worker::Worker;

#[derive(Debug)]
pub struct Server {
    me: usize,
    workers: Vec<Worker>,
    ipv4_addr: Ipv4Addr,
}


impl Server {
    pub fn new(server_id: usize, worker_size: usize, ipv4_addr: Ipv4Addr) -> Self {
        // todo: init workers
        Self { me: server_id, workers: vec![], ipv4_addr }
    }
}


pub fn say_hello_from_server(caller: &str) {
    println!("{}, Hello from the server mod!", caller)
}