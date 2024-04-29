use std::fmt;
use std::fmt::Debug;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::{Arc};
use tokio::time::{sleep, Duration};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use crate::server::client::Client;
use crate::server::worker::Worker;
use crate::server::msg::{AddRequest, AddResponse, PingRequest, PingResponse, Request, Response};
use tokio::runtime::Runtime;
use etcd_client::Client as EtcdClient;

const MAX_PACKET_BUFFER_SIZE: usize = 1452;

const ETCD_ADDR: &str = "http://127.0.0.1:2379";

pub struct Server {
    pub me: usize,
    workers: Vec<Worker>,
    pub ipv4_addr: Ipv4Addr,
    pub port: u16,
    pub peers: Vec<Client>,
    pub etcd_cli: Option<EtcdClient>,
    pub runtime: Runtime,
}

const DEFAULT_PORT: u16 = 9527;

pub type SharedServer = Arc<Mutex<Server>>;

impl Server {
    pub fn new(server_id: usize, _worker_size: usize, ipv4_addr: Ipv4Addr, world_size: usize) -> Self {
        // todo: init workers
        let rt = Runtime::new().unwrap();
        let mut server = Server {
            me: server_id,
            workers: Vec::new(),
            ipv4_addr,
            port: DEFAULT_PORT,
            peers: Vec::new(),
            etcd_cli: None,
            runtime: rt,
        };
        println!("world_size: {}", world_size);
        for i in 0..world_size {
            let mut peer = Client::new(i);
            if i == server_id {
                peer.socket_addr = server.socket_addr_str();
            }
            server.peers.push(peer);
        }
        server.runtime.block_on(async {
            let client = EtcdClient::connect([ETCD_ADDR], None).await.
                expect("connect to etcd server");
            server.etcd_cli = Option::from(client);
        });

        server.port = server.port + (server_id as u16);
        server
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

    fn socket_addr_str(&self) -> String {
        self.socket_addr().to_string()
    }

    fn etcd_key(&self) -> String {
        format!("server{}", self.me)
    }

    pub async fn register_in_etcd(&mut self) {
        let k = self.etcd_key();
        let v = self.socket_addr_str();
        let cli = self.etcd_cli.as_mut().unwrap();
        cli.put(k, v, None).await.expect("put kv");
        println!("success to register in etcd");
    }

    pub async fn get_etcd_value(&mut self, k: String) -> Option<String> {
        let cli = self.etcd_cli.as_mut().unwrap();
        let resp = cli.get(k.clone(), None).await.unwrap();
        let kvs = resp.kvs();
        if let Some(kv) = kvs.first() {
            Option::from(kv.value_str().unwrap().to_string())
        } else {
            None
        }
    }

    pub async fn config_peers(&mut self) {
        for i in 0..self.peers.len() {
            let k = self.peers[i].etcd_key();
            let mut v = self.get_etcd_value(k.clone()).await;
            while v.is_none() {
                sleep(Duration::from_millis(200)).await;
                v = self.get_etcd_value(k.clone()).await;
            }
            self.peers[i].socket_addr = v.unwrap();
        }
        for x in &self.peers {
            println!("peer.{} addr: {}", x.server_id, x.socket_addr);
        }
    }
}

pub async fn start_udp_service(server: Arc<Mutex<Server>>) {
    println!("I'm No. {} server. About me: {:?}", server.lock().await.me, server);
    let socket = UdpSocket::bind(server.lock().await.socket_addr()).await.unwrap();
    let r = Arc::new(socket);
    loop {
        let mut buf = [0; MAX_PACKET_BUFFER_SIZE];
        let tx = r.clone();
        let (_, addr) = r.recv_from(&mut buf).await.unwrap();
        let ss = server.clone();
        tokio::spawn(async move {
            let req: Request = bincode::deserialize(&buf).unwrap();
            let _ = ss.lock().await.handle_request(req);
            tx.send_to(&buf, &addr).await.unwrap();
        });
    }
}


pub fn say_hello_from_server(caller: &str) {
    println!("{}, Hello from the server mod!", caller)
}


impl Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Server").
            field("me", &self.me).
            finish()
    }
}
