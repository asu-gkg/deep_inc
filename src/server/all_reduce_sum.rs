use std::sync::Arc;
use tch::Tensor;
use tokio::sync::Mutex;
use crate::server::msg::{AllReduceSumOpRequest, Request, Response};
use crate::server::server::{MAX_PACKET_BUFFER_SIZE, Server};

impl Server {
    pub async fn all_reduce_sum(&self, tensor: Arc<Tensor>) -> Tensor {
        let agg = &self.agg_lst[self.agg_id];
        let shared_socket = agg.socket.clone().unwrap().clone();
        let socket = shared_socket.lock().await;
        let req = Request::AllReduceSumOp(AllReduceSumOpRequest::new(self.me, tensor));
        println!("agg.socket_addr: {}", agg.socket_addr);
        let data = bincode::serialize(&req).unwrap();
        socket.send(&data).await.unwrap();
        let mut buf = vec![0u8; MAX_PACKET_BUFFER_SIZE];
        socket.recv(&mut buf).await.unwrap();
        let resp: Response = bincode::deserialize(&buf).unwrap();
        match resp {
            Response::AllReduceSumOp(resp) => {
                println!("resp: {:?}", resp);
                return resp.tensor;
            }
            _ => { todo!("impl it") }
        }
    }
}