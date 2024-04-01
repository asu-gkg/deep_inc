#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use crate::config::config;
    use crate::config::config::make_local_server;
    use std::time::Instant;

    // #[test]
    #[test]
    fn test_config() {
        config::say_hello();
        let local_server = make_local_server(0, 1);
        let future = local_server.start_udp_service();
        block_on(future);
    }

    #[test]
    fn test_config_future() {
        // 经过测试, 用future不如用.await的方法快
        let start = Instant::now(); // 开始计时

        config::say_hello();
        let local_server = make_local_server(0, 1);
        let future = local_server.start_udp_service_future();
        block_on(future);

        let duration = start.elapsed(); // 计算持续时间
        println!("Tokio task took: {:?}", duration);
    }

    #[tokio::test]
    async fn test_config_tokio() {
        let start = Instant::now(); // 开始计时

        let local_server = make_local_server(0, 1);
        local_server.start_udp_service_tokio().await;

        let duration = start.elapsed(); // 计算持续时间
        println!("Tokio task took: {:?}", duration);
    }
}