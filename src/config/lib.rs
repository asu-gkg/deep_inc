#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, UdpSocket};
    use std::time::{Duration};
    use tokio::time::timeout;
    use crate::config::config::Config;
    use crate::server::server::Server;

    #[tokio::test]
    async fn test_config_tokio() {
        let local_server = Server::new(0, 1, Ipv4Addr::new(127, 0, 0, 1));
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
            let conf = Config::new(true, 0);
            match timeout(duration, conf.server.start_udp_service_tokio()).await {
                Ok(_) => { panic!("It shouldn't happen.") }
                Err(_) => { println!("After 3 seconds, server{} exited.", conf.server.me) }
            }
        });

        // config a client
        tokio::spawn(async {
            let socket = UdpSocket::bind("0.0.0.0:9577").unwrap();
            socket.connect("0.0.0.0:9527").expect("socket connect fail");
            let data = b"Hello, server! I'm test client!";
            socket.send(data).expect("send data fail");
            println!("client sends a msg to server");
        });

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}