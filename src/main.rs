pub mod config;
pub mod server;
use tch::{Device, Tensor};

const CALLER: &str = "Main";
fn main() {
    println!("Hello, world!");
    config::config::say_hello();
    server::server::say_hello_from_server(CALLER);

    let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();
    let t = t.to_device(Device::cuda_if_available());
    t.print();
    println!("cuda is {:?}", Device::cuda_if_available());


}
