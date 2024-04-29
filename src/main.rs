pub mod config;
pub mod server;

use etcd_client::Client;
use crate::config::config::Config;
use crate::server::say_hello_from_server;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    config::config::_say_hello();
    const CALLER: &str = "Main";
    say_hello_from_server(CALLER);


    let conf = Config::new_agg(0, 2);


}
