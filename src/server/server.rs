use std::fmt::Debug;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::os::unix::net::SocketAddr;
use log::log;
use crate::server::worker::Worker;

#[derive(Debug)]
pub struct Server {
    me: usize,
    workers: Vec<Worker>,
    ipv4_addr: Ipv4Addr,
    port: u16,
}

const DEFAULT_PORT: u16 = 9527;

impl Server {
    pub fn new(server_id: usize, worker_size: usize, ipv4_addr: Ipv4Addr) -> Self {
        // todo: init workers
        Self { me: server_id, workers: vec![], ipv4_addr, port: DEFAULT_PORT }
    }

    pub async fn start_udp_service(&self) {
        println!("I'm No. {} server. About me: {:?}", self.me, self);
        let socket = UdpSocket::bind(self.socket_addr());
        // todo
    }

    fn do_something() {
        for i in 0..100 {
            print!("1")
        }
        print!("\n");
    }
    pub async fn start_udp_service_future(&self) {
        println!("I'm No. {} server. About me: {:?}", self.me, self);
        let socket = UdpSocket::bind(self.socket_addr());
        Self::do_something();
    }

    pub async fn start_udp_service_await(&self) {
        println!("I'm No. {} server. About me: {:?}", self.me, self);
        let socket = UdpSocket::bind(self.socket_addr());
        Self::do_something();
    }
    pub async fn start_udp_service_tokio(&self) {
        println!("I'm No. {} server. About me: {:?}", self.me, self);
        let socket = UdpSocket::bind(self.socket_addr());

        Self::do_something();
    }

    fn socket_addr(&self) -> SocketAddrV4 {
        SocketAddrV4::new(self.ipv4_addr, self.port)
    }
}


pub fn say_hello_from_server(caller: &str) {
    println!("{}, Hello from the server mod!", caller)
}