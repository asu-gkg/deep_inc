#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use std::time::{Duration};
    use crate::server::server::Server;

    #[tokio::test]
    async fn test_config_tokio() {
        let local_server = Server::new(0, 1, Ipv4Addr::new(0, 0, 0, 0));
        tokio::spawn(async move {
            local_server.start_udp_service_tokio().await;
        });


        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}