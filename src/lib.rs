mod server;
mod config;

#[cfg(test)]
mod tests {
    use crate::config::config;

    #[test]
    fn test_udp_comm() {
        println!("hello")
    }
}