pub mod config;
pub mod server;

use crate::config::config::Config;
use crate::server::say_hello_from_server;

use tch::{Device, Kind, Tensor};

fn synthetic_data(w: Tensor, b: Tensor, num_examples: usize) -> (Tensor, Tensor) {
    println!("w.size: {:?}, w.dim: {}", w.size(), w.size()[0]);
    let X = Tensor::randn(&[1, w.size()[0]],
                          (Kind::Float, Device::Cpu));
    println!("X: {}", X);
    let mut y = &X.matmul(&w) + b;
    println!("y: {}", y);
    let mut tmp = y.copy();

    println!("tmp.normal: {}", tmp.normal_(0.0, 0.1));
    let y = &y.copy() + tmp;
    (X, y.reshape([-1, 1]))
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    config::config::_say_hello();
    const CALLER: &str = "Main";
    say_hello_from_server(CALLER);


    let true_w = Tensor::from_slice(&[2.0, -3.4]).to_kind(Kind::Float);
    println!("true_w: {}", true_w);
    let true_b = Tensor::from(3.2);
    let (features, labels) = synthetic_data(true_w, true_b, 10);


    println!("features: {}", features);
    println!("labels: {}", labels);
}
