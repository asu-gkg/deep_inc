use std::sync::Arc;
use std::time::Duration;
use etcd_client::GetOptions;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::time::sleep;
use crate::server::client::Client;
use crate::server::{etcd_key, get_server_id};
use crate::server::server::Role::{_Agg, _Worker};
use crate::server::server::{AGG_ETCD_KEY, Server, WORKER_ETCD_KEY};

impl Server {
    fn etcd_key(&self) -> String {
        etcd_key(self.role.unwrap(), self.me)
    }

    pub async fn register_in_etcd(&mut self) {
        let k = self.etcd_key();
        let v = self.socket_addr_str();
        let cli = self.etcd_cli.as_mut().unwrap();
        cli.put(k, v, None).await.expect("put kv");
        println!("success to register in etcd");
    }

    pub async fn get_etcd_value(&mut self, k: String) -> Option<String> {
        let cli = self.etcd_cli.as_mut().unwrap();
        let resp = cli.get(k.clone(), None).await.unwrap();
        let kvs = resp.kvs();
        if let Some(kv) = kvs.first() {
            Option::from(kv.value_str().unwrap().to_string())
        } else {
            None
        }
    }

    pub async fn get_etcd_value_with_prefix(&mut self, k: String, expect_len: Option<usize>) -> Vec<(String, String)> {
        let cli = self.etcd_cli.as_mut().unwrap();
        let mut resp = &cli.get(k.clone(), Some(GetOptions::new().with_prefix())).await.unwrap();
        let mut kvs = resp.kvs();
        let mut ret = Vec::new();
        println!("k: {}", k);
        println!("kvs.len: {}", kvs.len());
        if expect_len.is_some() && kvs.len() != expect_len.unwrap() {
            return ret;
        }
        for x in kvs {
            ret.push((String::from(x.key_str().unwrap()), String::from(x.value_str().unwrap())));
        }
        ret
    }

    pub async fn config_peers(&mut self) {
        print!("finding peers..");
        for i in 0..self.peers.len() {
            let k = self.peers[i].etcd_key();
            let mut v = self.get_etcd_value(k.clone()).await;
            while v.is_none() {
                sleep(Duration::from_millis(200)).await;
                v = self.get_etcd_value(k.clone()).await;
                print!(".");
            }
            self.peers[i].socket_addr = v.unwrap();
        }
        println!();
        println!("self.peers.len: {}", self.peers.len());
        for x in &self.peers {
            println!("peer.{} addr: {}", x.server_id, x.socket_addr);
        }
    }

    pub async fn config_workers_for_agg(&mut self) {
        if self.role != Some(_Agg) {
            panic!("Only agg node could config workers.")
        }
        for i in 0..self.world_size {
            self.workers.push(Client::new(i));
        }
        let mut kvs = self.get_etcd_value_with_prefix(String::from(WORKER_ETCD_KEY), Some(self.world_size)).await;
        println!("finding workers..");
        while kvs.len() != self.world_size {
            print!(".");
            sleep(Duration::from_millis(200)).await;
            kvs = self.get_etcd_value_with_prefix(String::from(WORKER_ETCD_KEY), Some(self.world_size)).await;
        }
        println!();
        for x in kvs {
            let worker_sid = get_server_id(x.0);
            let v = x.1;
            println!("worker_sid: {}, v: {}", worker_sid, v);
            self.workers[worker_sid].socket_addr = v;
        }
    }

    pub async fn config_agg_for_worker(&mut self) {
        if self.role != Some(_Worker) {
            panic!("This node doesn't have to config agg.")
        }
        for i in 0..self.agg_size {
            self.agg_lst.push(Client::new_agg(i));
        }
        let mut kvs = self.get_etcd_value_with_prefix(String::from(AGG_ETCD_KEY), Some(self.agg_size)).await;
        println!("kvs.len(): {}, self.agg_size : {}", kvs.len(), self.agg_size);
        println!("finding agg..");
        while kvs.len() != self.agg_size {
            print!(".");
            sleep(Duration::from_millis(200)).await;
            kvs = self.get_etcd_value_with_prefix(String::from(AGG_ETCD_KEY), Some(self.agg_size)).await;
        }
        println!();
        for x in kvs {
            let agg_sid = get_server_id(x.0);
            let v = x.1;
            println!("agg_sid: {}, v: {}", agg_sid, v);
            self.agg_lst[agg_sid].socket_addr = v;
            let sock = UdpSocket::bind(self.socket_addr().to_string()).await.unwrap();
            sock.connect(self.agg_lst[agg_sid].socket_addr.clone()).await.unwrap();
            self.agg_lst[agg_sid].socket = Some(Arc::new(Mutex::new(sock)));
        }
        println!("success to get agg list");
    }
}