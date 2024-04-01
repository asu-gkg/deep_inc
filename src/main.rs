mod config;
mod server;

fn main() {
    println!("Hello, world!");
    config::config::say_hello();
    server::say_hello_from_server();
}
