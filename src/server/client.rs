#[derive(Debug)]
pub struct Client {
    pub server_id: usize,
    pub socket_addr: String,

}

impl Client {
    pub fn new(server_id: usize) -> Self {
        Self { server_id, socket_addr: "".to_string() }
    }


    pub fn new_agg() -> Self {
        Self { server_id: 0, socket_addr: "".to_string() }
    }

    pub fn etcd_key(&self) -> String {
        format!("server{}", self.server_id)
    }
}


