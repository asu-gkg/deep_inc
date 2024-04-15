pub mod config;
pub mod server;

use tch::{Device, Kind, Tensor};

const CALLER: &str = "Main";

fn f(a: &Tensor) -> Tensor {
    let mut b: Tensor = 2 * a;
    println!("-----loop start-----");
    // 这里计算的是L1范数
    while b.norm().double_value(&[]) < 1000.0 {
        println!("b: {}", b);
        println!("b.norm: {}", b.norm());
        println!("b.norm==b: {}", b.norm() == b.absolute());
        b = &b * 2;
    }
    let mut c = Tensor::new();
    if b.sum(Kind::Float).double_value(&[]) > 0.0 {
        c = &b * 1;
    } else {
        c = &b * 100;
    }
    c
}

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

    x.zero_grad();
    y = &x * &x;
    let u = y.detach();
    let z = &u * &x;
    z.sum(Kind::Float).backward();
    println!("x.grad: {}", x.grad());
    println!("u: {}", u);
    println!("u==x: {}", u == x.grad());

    let _ = x.grad().zero_();
    y.sum(Kind::Float).backward();
    println!("x.grad == 2*x: {}", 2 * &x == x.grad());

    let a = Tensor::randn([], (Kind::Float, Device::Cpu));
    let a = a.set_requires_grad(true);
    println!("a: {}", a);
    let d = f(&a);
    println!("d: {}", d);
    d.backward();
    println!("a.grad: {}", a.grad());
    println!("a.grad == d/a: {}", a.grad() == d / a);
}

fn main() {
    println!("Hello, world!");
    config::config::say_hello();
    server::server::say_hello_from_server(CALLER);

    auto_grad();
}
