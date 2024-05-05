use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use crate::server::etcd_key;
use crate::server::server::Role;

#[derive(Debug)]
pub struct Client {
    pub server_id: usize,
    pub socket_addr: String,
    pub role: Role,
    pub socket: Option<Arc<Mutex<UdpSocket>>>,
}

impl Client {
    pub fn new(server_id: usize) -> Self {
        Self { server_id, socket_addr: "".to_string(), role: Role::_Worker, socket: None }
    }


    pub fn new_agg(server_id: usize) -> Self {
        Self { server_id: 0, socket_addr: "".to_string(), role: Role::_Agg, socket: None }
    }

    pub fn etcd_key(&self) -> String {
        etcd_key(self.role, self.server_id)
    }
}


