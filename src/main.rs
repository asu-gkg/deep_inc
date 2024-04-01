pub mod config;
pub mod server;

const CALLER: &str = "Main";
fn main() {
    println!("Hello, world!");
    config::config::say_hello();
    server::server::say_hello_from_server(CALLER);
}
