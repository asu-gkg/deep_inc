#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use futures::executor::block_on;
    use crate::config::config;
    use crate::config::config::make_local_server;
    use std::time::{Duration, Instant};

    // #[test]
    #[test]
    fn test_config() {
        config::say_hello();
        let local_server = make_local_server(0, 1);
        let future = local_server.start_udp_service();
        block_on(future);
        sleep(Duration::from_secs(2));
    }

    #[test]
    fn test_config_future() {
        let start = Instant::now();

        config::say_hello();
        let local_server = make_local_server(0, 1);
        let future = local_server.start_udp_service_future();
        block_on(future);

        let duration = start.elapsed();
        println!("Tokio task took: {:?}", duration);
        sleep(Duration::from_secs(2));
    }

    #[tokio::test]
    async fn test_config_await() {
        tokio::time::pause();
        let start = Instant::now();

        let local_server = make_local_server(0, 1);
        local_server.start_udp_service_await().await;

        let duration = start.elapsed();
        println!("Tokio task took: {:?}", duration);
    }

    #[tokio::test]
    async fn test_config_tokio() {
        let local_server = make_local_server(0, 1);
        tokio::spawn(async move {
            let start = Instant::now();
            local_server.start_udp_service_tokio().await;
            let duration = start.elapsed();
            println!("Tokio task took: {:?}", duration);
        });
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}