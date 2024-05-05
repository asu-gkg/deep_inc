use std::fmt;
use std::fmt::Debug;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::{Arc};
use tokio::time::{sleep, Duration};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use crate::server::client::Client;
use crate::server::msg::{AddRequest, AddResponse, AllReduceSumOpRequest, PingRequest, PingResponse, Request, Response};
use tokio::runtime::Runtime;
use etcd_client::{Client as EtcdClient, GetOptions};
use tch::Tensor;
use crate::server::{etcd_key, get_server_id};
use crate::server::server::Role::{_Agg, _Worker};

const MAX_PACKET_BUFFER_SIZE: usize = 1452;

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
        }
    }

    fn handle_add(&self, req: AddRequest) -> Option<AddResponse> {
        // add code here
        println!("recv add req, a: {}, b: {}", req.a, req.b);
        None
    }

    pub async fn all_reduce_sum(&self, mut tensor: Arc<Tensor>) {
        let agg = &self.agg_lst[self.agg_id];
        let shared_socket = agg.socket.clone().unwrap().clone();
        let socket = shared_socket.lock().await;
        let req = AllReduceSumOpRequest::new(self.me, tensor);
        let data = bincode::serialize(&req).unwrap();
        socket.send(&data).await.unwrap();
        loop {}
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
        etcd_key(self.role.unwrap(), self.me)
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

    pub async fn get_etcd_value_with_prefix(&mut self, k: String, expect_len: Option<usize>) -> Vec<(String, String)> {
        let cli = self.etcd_cli.as_mut().unwrap();
        let mut resp = &cli.get(k.clone(), Some(GetOptions::new().with_prefix())).await.unwrap();
        let mut kvs = resp.kvs();
        let mut ret = Vec::new();
        println!("k: {}", k);
        println!("kvs.len: {}", kvs.len());
        if expect_len.is_some() && kvs.len() != expect_len.unwrap() {
            return ret;
        }
        for x in kvs {
            ret.push((String::from(x.key_str().unwrap()), String::from(x.value_str().unwrap())));
        }
        ret
    }

    pub async fn config_peers(&mut self) {
        print!("finding peers..");
        for i in 0..self.peers.len() {
            let k = self.peers[i].etcd_key();
            let mut v = self.get_etcd_value(k.clone()).await;
            while v.is_none() {
                sleep(Duration::from_millis(200)).await;
                v = self.get_etcd_value(k.clone()).await;
                print!(".");
            }
            self.peers[i].socket_addr = v.unwrap();
        }
        println!();
        println!("self.peers.len: {}", self.peers.len());
        for x in &self.peers {
            println!("peer.{} addr: {}", x.server_id, x.socket_addr);
        }
    }

    pub async fn config_workers_for_agg(&mut self) {
        if self.role != Some(_Agg) {
            panic!("Only agg node could config workers.")
        }
        for i in 0..self.world_size {
            self.workers.push(Client::new(i));
        }
        let mut kvs = self.get_etcd_value_with_prefix(String::from(WORKER_ETCD_KEY), Some(self.world_size)).await;
        println!("finding workers..");
        while kvs.len() != self.world_size {
            print!(".");
            sleep(Duration::from_millis(200)).await;
            kvs = self.get_etcd_value_with_prefix(String::from(WORKER_ETCD_KEY), Some(self.world_size)).await;
        }
        println!();
        for x in kvs {
            let worker_sid = get_server_id(x.0);
            let v = x.1;
            println!("worker_sid: {}, v: {}", worker_sid, v);
            self.workers[worker_sid].socket_addr = v;
        }
    }

    pub async fn config_agg_for_worker(&mut self) {
        if self.role != Some(_Worker) {
            panic!("This node doesn't have to config agg.")
        }
        for i in 0..self.agg_size {
            self.agg_lst.push(Client::new_agg(i));
        }
        let mut kvs = self.get_etcd_value_with_prefix(String::from(AGG_ETCD_KEY), Some(self.agg_size)).await;
        println!("kvs.len(): {}, self.agg_size : {}", kvs.len(), self.agg_size);
        println!("finding agg..");
        while kvs.len() != self.agg_size {
            print!(".");
            sleep(Duration::from_millis(200)).await;
            kvs = self.get_etcd_value_with_prefix(String::from(AGG_ETCD_KEY), Some(self.agg_size)).await;
        }
        println!();
        for x in kvs {
            let agg_sid = get_server_id(x.0);
            let v = x.1;
            println!("agg_sid: {}, v: {}", agg_sid, v);
            self.agg_lst[agg_sid].socket_addr = v;
            let sock = UdpSocket::bind(self.socket_addr().to_string()).await.unwrap();
            sock.connect(self.agg_lst[agg_sid].socket_addr.clone()).await.unwrap();
            self.agg_lst[agg_sid].socket = Some(Arc::new(Mutex::new(sock)));
        }
        println!("success to get agg list");
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
