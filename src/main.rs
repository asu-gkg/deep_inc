pub mod config;
pub mod server;

use crate::config::config::Config;
use crate::server::say_hello_from_server;

use tch::{Device, Kind, Tensor};
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    config::config::_say_hello();
    const CALLER: &str = "Main";
    say_hello_from_server(CALLER);
    let true_w = Tensor::from_slice(&[2.0, -3.4]).to_kind(Kind::Float);
    println!("true_w: {}", true_w);

    let mut conf = Config::new_agg(0, 2);
    {
        let s = conf.server.as_mut().unwrap();
        s.config_etcd().await;
        s.register_in_etcd().await;
        s.config_workers_for_agg().await;
        s.start_udp_service_tokio().await;
    }
}
