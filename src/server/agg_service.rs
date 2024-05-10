use std::sync::Arc;
use tokio::net::UdpSocket;
use crate::server::msg::{AllReduceSumOpResponse, Request, Response};
use crate::server::server::{MAX_PACKET_BUFFER_SIZE, Server};

impl Server {
    pub async fn start_udp_service_tokio(&self) {
        println!("I'm No. {} server. About me: {:?}, socket_addr: {}", self.me, self, self.socket_addr_str());
        let socket = UdpSocket::bind(self.socket_addr()).await.unwrap();
        let r = Arc::new(socket);
        let me = self.me;
        loop {
            // fixme: didn't add restriction to the buffer. + flow control + congestion control
            let mut buf = [0; MAX_PACKET_BUFFER_SIZE];
            let tx = r.clone();
            let (_, addr) = r.recv_from(&mut buf).await.unwrap();
            let req: Request = bincode::deserialize(&buf).unwrap();
            println!("req: {:?}, addr: {:?}", req, addr);
            match req {
                Request::AllReduceSumOp(req) => {
                    println!("req: {:?}", &req);
                    let mut record = self.all_reduce_state.lock().await;
                    record.insert(req.server_id, req.tensor);
                    println!("world_size: {}, now recv {} tensor", self.world_size, record.len());

                    if record.len() == self.world_size {
                        let mut result = record.get_key_value(&0).unwrap().1.zero();
                        for x in record.iter() {
                            result = result + x.1;
                        }
                        // println!("{:?}", result);
                        record.clear();
                        let resp = Response::AllReduceSumOp(AllReduceSumOpResponse::new(me, result));
                        self.write_all(tx, resp);
                    }
                }
                _ => { panic!("Shouldn't be here.") }
            }
        }
    }
}