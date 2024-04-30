use std::string::ToString;
use pyo3::{pyclass, pymethods, pymodule, PyResult, Python};
use pyo3::prelude::PyModule;
// use tch::{Kind, Tensor};
use crate::config::config::Config;
use crate::server::server::Role::_Worker;

const REDUCE_OP_SUM: &str = "reduce_op_sum";

#[pyclass]
struct IncHandle {}

#[pymethods]
impl IncHandle {
    #[new]
    fn new() -> Self {
        println!("try init py_handle");
        Self {}
    }

    fn init_process_group(&self, rank: usize, world_size: usize) -> PyResult<()> {
        let mut conf = Config::new(false, rank, world_size);
        println!("try start_udp_service_tokio");

        pyo3_asyncio::tokio::get_runtime().block_on(async move {
            let s = conf.server.as_mut().unwrap();
            s.set_role(_Worker);
            s.config_etcd().await;
            s.register_in_etcd().await;
            s.config_peers().await;
            s.config_agg_for_worker().await;

            tokio::spawn(async move {
                let s = conf.server.as_mut().unwrap();
                s.start_udp_service_tokio().await;
            });
        });
        Ok(())
    }

    fn _all_reduce(&self, addr: usize, size: usize, shape: Vec<i64>, data_type: String, op: String) -> PyResult<()> {
        if op != REDUCE_OP_SUM {
            panic!("impl it");
        }
        // unsafe {
        //     println!("data_type: {}", data_type);
        //     let data = std::slice::from_raw_parts(addr as *const u8, size);
        //     let tensor = Tensor::from_data_size(data, &shape, Kind::Float);
        //
        //     let sum = tensor.sum(Kind::Float);
        //
        //     println!("Sum of elements: {:?}", sum);
        //     panic!("hi");
        // }
        pyo3_asyncio::tokio::get_runtime().block_on(async move {});
        Ok(())
    }
}

#[pymodule]
fn deep_inc(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IncHandle>()?;
    Ok(())
}