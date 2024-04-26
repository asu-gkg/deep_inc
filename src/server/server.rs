use std::fmt::Debug;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::{Arc};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

use crate::server::worker::Worker;
use crate::server::msg::{AddRequest, AddResponse, PingRequest, PingResponse, Request, Response};

const MAX_PACKET_BUFFER_SIZE: usize = 1452;

#[derive(Debug)]
pub struct Server {
    pub me: usize,
    workers: Vec<Worker>,
    pub ipv4_addr: Ipv4Addr,
    pub port: u16,
    pub peers: Vec<Server>,
}

const DEFAULT_PORT: u16 = 9527;

pub type SharedServer = Arc<Mutex<Server>>;

impl Server {
    pub fn new(server_id: usize, worker_size: usize, ipv4_addr: Ipv4Addr) -> Self {
        // todo: init workers
        Self { me: server_id, workers: vec![], ipv4_addr, port: DEFAULT_PORT, peers: vec![] }
    }

    fn handle_request(&self, req: Request) -> Response {
        match req {
            Request::Add(req) => {
                Response::Add(self.handle_add(req).unwrap())
            }
            Request::Ping(req) => {
                Response::Ping(self.handle_ping(req).unwrap())
            }
        }
    }

    fn handle_add(&self, req: AddRequest) -> Option<AddResponse> {
        // add code here
        println!("recv add req, a: {}, b: {}", req.a, req.b);
        None
    }

    fn handle_ping(&self, req: PingRequest) -> Option<PingResponse> {
        // add code here
        None
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

pub async fn start_udp_service(server: Arc<Mutex<Server>>) {
    println!("I'm No. {} server. About me: {:?}", server.lock().await.me, server);
    let socket = UdpSocket::bind(server.lock().await.socket_addr()).await.unwrap();
    let r = Arc::new(socket);
    loop {
        let mut buf = [0; MAX_PACKET_BUFFER_SIZE];
        let tx = r.clone();
        let (len, addr) = r.recv_from(&mut buf).await.unwrap();
        let ss = server.clone();
        tokio::spawn(async move {
            let req: Request = bincode::deserialize(&buf).unwrap();
            let resp = ss.lock().await.handle_request(req);
            tx.send_to(&buf, &addr).await.unwrap();
        });
    }
}


pub fn say_hello_from_server(caller: &str) {
    println!("{}, Hello from the server mod!", caller)
}
