mod server;
mod config;

#[cfg(test)]
mod tests {
    use crate::config::config;

    #[test]
    fn test_udp_comm() {
        println!("hello")
    }

    #[test]
    fn test_config() {
        config::say_hello();


    }
}