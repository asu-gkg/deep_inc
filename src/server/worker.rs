#[derive(Debug)]
pub struct Worker {
    me: usize,

}

impl Worker {
    pub fn new(id: usize) -> Self {
        Self { me: id }
    }

    pub fn add(&self, sender_id: usize, a: i32, b: i32) -> i32 {
        a + b
    }
}