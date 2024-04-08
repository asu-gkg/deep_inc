use std::fmt::Debug;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::{Arc};
use tokio::net::UdpSocket;

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

        // todo
    }

    pub async fn start_udp_service_tokio(&self) {
        println!("I'm No. {} server. About me: {:?}", self.me, self);
        let socket = UdpSocket::bind(self.socket_addr()).await.unwrap();
        let r = Arc::new(socket);
        let s = r.clone();

        let mut buf = [0; 1024]; // fixme: if we only have one buffer, it would cause high-computing time
        loop {
            let (len, addr) = r.recv_from(&mut buf).await.unwrap();
            let bytes = buf.to_vec();
            let s = s.clone();
            tokio::spawn(async move {
                println!("{:?} bytes received from {:?}", len, addr);
                s.send_to(&bytes, &addr).await.unwrap();
            });
        }
    }

    fn socket_addr(&self) -> SocketAddrV4 {
        SocketAddrV4::new(self.ipv4_addr, self.port)
    }
}


pub fn say_hello_from_server(caller: &str) {
    println!("{}, Hello from the server mod!", caller)
}