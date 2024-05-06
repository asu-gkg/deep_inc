use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::ops::Add;
use std::sync::{Arc};
use tokio::time::{sleep, Duration, timeout};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use crate::server::client::Client;
use crate::server::msg::{AddRequest, AddResponse, AllReduceSumOpRequest, AllReduceSumOpResponse, PingRequest, PingResponse, Request, Response};
use tokio::runtime::Runtime;
use etcd_client::{Client as EtcdClient, GetOptions};
use tch::Tensor;
use crate::server::{etcd_key, get_server_id};
use crate::server::server::Role::{_Agg, _Worker};

pub(crate) const MAX_PACKET_BUFFER_SIZE: usize = 1452;

const ETCD_ADDR: &str = "http://127.0.0.1:2379";

pub(crate) const WORKER_ETCD_KEY: &str = "worker";

pub(crate) const AGG_ETCD_KEY: &str = "agg";

pub(crate) const ROOT_ETCD_KEY: &str = "root";

#[derive(PartialEq, Eq)]
#[derive(Debug, Copy, Clone)]
pub enum Role {
    _Worker,
    _Agg,
    _Root,
}

pub struct Server {
    pub me: usize,
    pub workers: Vec<Client>,
    pub ipv4_addr: Ipv4Addr,
    pub port: u16,
    pub peers: Vec<Client>,
    pub etcd_cli: Option<EtcdClient>,
    pub runtime: Runtime,
    pub role: Option<Role>,
    pub agg_lst: Vec<Client>,
    pub world_size: usize,
    pub agg_id: usize,
    pub agg_size: usize,

    pub(crate) all_reduce_state: Arc<Mutex<HashMap<usize, Tensor>>>,
}

const DEFAULT_PORT: u16 = 9527;

pub type SharedServer = Arc<Mutex<Server>>;

impl Server {
    pub fn new(server_id: usize, _worker_size: usize, ipv4_addr: Ipv4Addr, world_size: usize) -> Self {
        // todo: init workers
        let mut server = Server {
            me: server_id,
            workers: Vec::new(),
            ipv4_addr,
            port: DEFAULT_PORT,
            peers: Vec::new(),
            etcd_cli: None,
            runtime: Runtime::new().unwrap(),
            role: None,
            agg_lst: Vec::new(),
            agg_id: 0,
            world_size,
            agg_size: 1,
            all_reduce_state: Arc::new(Mutex::new(HashMap::new())),
        };
        println!("world_size: {}", world_size);
        for i in 0..world_size {
            let mut peer = Client::new(i);
            if i == server_id {
                peer.socket_addr = server.socket_addr_str();
            }
            server.peers.push(peer);
        }
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
            _ => { panic!("impl it") }
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


    pub(crate) fn socket_addr(&self) -> SocketAddrV4 {
        SocketAddrV4::new(self.ipv4_addr, self.port)
    }

    pub(crate) fn socket_addr_str(&self) -> String {
        self.socket_addr().to_string()
    }

    pub async fn config_etcd(&mut self) {
        let client = EtcdClient::connect([ETCD_ADDR], None).await.
            expect("connect to etcd server");
        self.etcd_cli = Option::from(client);
    }

    pub fn set_role(&mut self, role: Role) {
        self.role = Option::from(role);
        self.config_port();
    }
    pub fn config_port(&mut self) {
        if self.role == Some(_Worker) {
            self.port = self.port + (self.me as u16);
        }
        if self.role == Some(_Agg) {
            self.port = self.port + (self.me as u16) + 8_9_6_4;
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

impl Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Server").
            field("me", &self.me).
            field("role", &self.role).
            finish()
    }
}
