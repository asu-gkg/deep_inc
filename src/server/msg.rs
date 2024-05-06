use std::sync::Arc;
use serde_derive::{Deserialize, Serialize};
use tch::Tensor;


#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Add(AddRequest),
    Ping(PingRequest),
    AllReduceSumOp(AllReduceSumOpRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Add(AddResponse),
    Ping(PingResponse),
    AllReduceSumOp(AllReduceSumOpResponse),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRequest {
    pub sender_id: usize,
    pub receiver_id: usize,
    pub a: i32,
    pub b: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllReduceSumOpRequest {
    pub server_id: usize,
    #[serde(with = "tch_serde::serde_tensor")]
    pub tensor: Tensor,
}

impl AllReduceSumOpRequest {
    pub fn new(server_id: usize, tensor: Arc<Tensor>) -> Self {
        Self { server_id, tensor: tensor.copy() }
    }
}

impl AddRequest {
    pub fn new(sender_id: usize, receiver_id: usize, a: i32, b: i32) -> Self {
        Self { sender_id, receiver_id, a, b }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddResponse {
    pub sender_id: usize,
    pub receiver_id: usize,
    pub sum: i32,
}

#[derive(Serialize, Deserialize)]
struct SerializableTensor {
    data: Vec<f32>,
    shape: Vec<i64>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AllReduceSumOpResponse {
    pub agg_id: usize,
    #[serde(with = "tch_serde::serde_tensor")]
    pub tensor: Tensor,
}

impl AllReduceSumOpResponse {
    pub fn new(agg_id: usize, tensor: Tensor) -> Self {
        Self { agg_id, tensor }
    }
}


impl AddResponse {
    pub fn new(sender_id: usize, receiver_id: usize, sum: i32) -> Self {
        Self { sender_id, receiver_id, sum }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {
    pub sender_id: usize,
    pub receiver_id: usize,
}

impl PingRequest {
    pub fn new(sender_id: usize, receiver_id: usize) -> Self {
        Self { sender_id, receiver_id }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    pub sender_id: usize,
    pub receiver_id: usize,
    pub _str: String,
}

impl PingResponse {
    pub fn new(sender_id: usize, receiver_id: usize, _str: String) -> Self {
        Self { sender_id, receiver_id, _str }
    }
}
