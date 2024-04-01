use crate::server;

#[derive(Debug)]
struct Config {
    standalone: bool,
}

const CALLER: &str = "Config";

impl Config {
    pub fn new(standalone: bool) -> Self {
        Self {
            standalone
        }
    }
}

pub fn say_hello() {
    println!("Hello from the config mod!");
    server::server::say_hello_from_server(CALLER);
}

pub fn make_server() {

}