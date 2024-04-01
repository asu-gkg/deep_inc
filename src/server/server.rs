use std::net::IpAddr;
use crate::server::worker::Worker;

#[derive(Debug)]
struct Server {
    me: usize,
    workers: Vec<Worker>,
    ip_addr: IpAddr,
}


impl Server {
    pub fn new(server_id: usize, worker_size: usize, ip_addr: IpAddr) -> Self {
        // todo: init workers
        Self { me: server_id, workers: vec![], ip_addr }
    }
}


pub fn say_hello_from_server(caller: &str) {
    println!("{}, Hello from the server mod!", caller)
}