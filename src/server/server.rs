use std::fmt::Debug;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::{Arc};
use tokio::net::UdpSocket;

use crate::server::worker::Worker;

const MAX_PACKET_BUFFER_SIZE: usize = 1452;

#[derive(Debug)]
pub struct Server {
    pub me: usize,
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
        let server_id = self.me;
        loop {
            // fixme: didn't add restriction to the buffer. + flow control + congestion control
            let mut buf = [0; MAX_PACKET_BUFFER_SIZE];
            let s = r.clone();
            let (len, addr) = r.recv_from(&mut buf).await.unwrap();
            tokio::spawn(async move {
                println!("Server. {}: {:?} bytes received from {:?}", server_id, len, addr);
                s.send_to(&buf, &addr).await.unwrap();
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