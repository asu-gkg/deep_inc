use crate::server::etcd_key;
use crate::server::server::Role;

#[derive(Debug)]
pub struct Client {
    pub server_id: usize,
    pub socket_addr: String,
    pub role: Role,
}

impl Client {
    pub fn new(server_id: usize) -> Self {
        Self { server_id, socket_addr: "".to_string(), role: Role::_Worker }
    }


    pub fn new_agg(server_id: usize) -> Self {
        Self { server_id: 0, socket_addr: "".to_string(), role: Role::_Agg }
    }

    pub fn etcd_key(&self) -> String {
        etcd_key(self.role, self.server_id)
    }
}


