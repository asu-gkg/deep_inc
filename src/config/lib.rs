#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use crate::config::config;
    use crate::config::config::make_local_server;

    #[test]
    fn test_config() {
        config::say_hello();
        let local_server = make_local_server(0, 1);

        let future = local_server.start_udp_service();
        block_on(future);
    }
}