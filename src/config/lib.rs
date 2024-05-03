#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, UdpSocket};
    use std::sync::Arc;
    use std::time::{Duration};
    use tokio::time::timeout;
    use tokio::sync::{Mutex};
    use crate::config::config::Config;
    use crate::server::msg::AddRequest;
    use crate::server::server::{Server, start_udp_service};

    #[tokio::test]
    async fn test_config_tokio() {
        let local_server = Server::new(0, 1, Ipv4Addr::new(127, 0, 0, 1), 1);
        tokio::spawn(async move {
            local_server.start_udp_service_tokio().await;
        });
        tokio::time::sleep(Duration::from_secs(10)).await;
    }

    #[tokio::test]
    async fn test_ping_server() {
        // config a server
        tokio::spawn(async {
            let duration = Duration::from_secs(3);
            let mut conf = Config::new(true, 0, 1);
            match timeout(duration, conf.server.start_udp_service_tokio()).await {
                Ok(_) => { panic!("It shouldn't happen.") }
                Err(_) => { println!("After 3 seconds, server. {} exited.", conf.server.me) }
            }
        });

        // config a client
        tokio::spawn(async {
            let socket = UdpSocket::bind("0.0.0.0:9577").unwrap();
            socket.connect("0.0.0.0:9527").expect("socket connect fail");
            let data = b"Hello, server! I'm test client!";
            socket.send(data).expect("send data fail");
            println!("Client sends a msg to server");
        });

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    #[tokio::test]
    async fn test_add_rpc() {
        tokio::spawn(async {
            let duration = Duration::from_secs(3);
            let mut conf = Config::new(true, 0, 1);
            let server_id = conf.server.unwrap().lock().await.me;
            let shared_server = Arc::new(Mutex::new(conf.server.unwrap().lock().await.unwrap()));

            match timeout(duration, start_udp_service(shared_server)).await {
                Ok(_) => { panic!("It shouldn't happen.") }
                Err(_) => { println!("After 3 seconds, server. {} exited.", server_id) }
            }
        });

        tokio::spawn(async {
            let socket = UdpSocket::bind("0.0.0.0:9577").unwrap();
            socket.connect("0.0.0.0:9527").expect("socket connect fail");
            let req = AddRequest::new(0, 0, 1, 1);
            let data = bincode::serialize(&req).unwrap();
            socket.send(&data).expect("send data fail");
            println!("Client sends a msg to server");
        });

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}