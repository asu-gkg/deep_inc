pub mod server;
pub mod worker;
pub mod msg;
mod client;


pub fn say_hello_from_server(caller: &str) {
    println!("{}, Hello from the server mod!", caller)
}