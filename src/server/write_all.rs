use std::sync::Arc;
use tokio::net::UdpSocket;
use crate::server::msg::Response;
use crate::server::server::Server;

impl Server {
    pub fn write_all(&self, tx: Arc<UdpSocket>, resp: Response) {
        let workers = &self.workers;
        let buf = bincode::serialize(&resp).unwrap();
        for cli in workers {
            let addr = cli.socket_addr.clone();
            let tx = tx.clone();
            let buf = buf.clone();
            pyo3_asyncio::tokio::get_runtime().spawn(async move {
                tx.send_to(&buf, addr).await.expect("Fail to send resp.");
            });
        }
    }
}