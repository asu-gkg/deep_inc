use crate::server::server::Role;
use crate::server::server::Role::{_Agg, _Root, _Worker};

pub mod server;
pub mod worker;
pub mod msg;
mod client;
mod write_all;
mod agg_service;
mod register;
mod all_reduce_sum;


pub fn say_hello_from_server(caller: &str) {
    println!("{}, Hello from the server mod!", caller)
}

pub fn get_server_id(k: String) -> usize {
    let parts: Vec<&str> = k.split('/').collect();
    let num_str = parts.last().unwrap();
    num_str.parse().unwrap()
}

fn etcd_key(role: Role, sid: usize) -> String {
    if _Worker == role {
        return format!("{}/{}", server::WORKER_ETCD_KEY, sid);
    }
    if _Agg == role {
        return format!("{}/{}", server::AGG_ETCD_KEY, sid);
    }
    if _Root == role {
        return format!("{}/{}", server::ROOT_ETCD_KEY, sid);
    }
    panic!("shouldn't be here");
}