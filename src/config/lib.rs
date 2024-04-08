#[cfg(test)]
mod tests {
    use crate::config::config::make_local_server;
    use std::time::{Duration};

    // #[test]
    #[tokio::test]
    async fn test_config_tokio() {
        let local_server = make_local_server(0, 1);
        tokio::spawn(async move {
            local_server.start_udp_service_tokio().await;
        });


        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}