#[derive(Debug)]
pub struct Worker {
    me: usize,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        Self { me: id }
    }
}