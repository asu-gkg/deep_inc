use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Add(AddRequest),
    Ping(PingRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Add(AddResponse),
    Ping(PingResponse),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRequest {
    pub sender_id: usize,
    pub receiver_id: usize,
    pub a: i32,
    pub b: i32,
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
