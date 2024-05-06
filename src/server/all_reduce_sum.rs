use std::sync::Arc;
use tch::Tensor;
use crate::server::msg::{AllReduceSumOpRequest, Request};
use crate::server::server::Server;

impl Server {
    pub async fn all_reduce_sum(&self, mut tensor: Arc<Tensor>) {
        let agg = &self.agg_lst[self.agg_id];
        let shared_socket = agg.socket.clone().unwrap().clone();
        let socket = shared_socket.lock().await;
        let req = Request::AllReduceSumOp(AllReduceSumOpRequest::new(self.me, tensor));
        println!("agg.socket_addr: {}", agg.socket_addr);
        let data = bincode::serialize(&req).unwrap();
        socket.send(&data).await.unwrap();
        loop {}
    }
}