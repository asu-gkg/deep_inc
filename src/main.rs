pub mod config;
pub mod server;

use tch::{Device, Kind, Tensor};

const CALLER: &str = "Main";

// auto grad
fn auto_grad() {
    let mut x = Tensor::arange(4,
                               (Kind::Float, Device::Cuda(0))).
        set_requires_grad(true);

    println!("x: {}", x);
    println!("x.grad: {}", x.grad());

    let mut y: Tensor = 2 * &x.dot(&x);
    println!("y: {}", y);
    y.backward();
    println!("x.grad: {}", &x.grad());
    println!("x.grad == 4*x: {}", x.grad() == 4 * &x);


    x.zero_grad();
    println!("x.grad: {}", x.grad());
    y = x.sum(Kind::Float);
    y.backward();
    println!("x.grad: {}", x.grad());

    let grad = x.grad().zero_();
    println!("grad: {}", grad, );
    println!("x.grad: {}", x.grad());

    y = &x * &x;
    y.sum(Kind::Float).backward();
    println!("x.grad: {}", x.grad());
}

fn main() {
    println!("Hello, world!");
    config::config::say_hello();
    server::server::say_hello_from_server(CALLER);

    auto_grad();
}
