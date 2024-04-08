enum Operation {
    Reduce,
    AllReduce,
}

pub struct RequestMsg {
    _type: Operation,
    tensor: Vec<f32>,
}