use std::ops::Deref;
use std::string::ToString;
use std::sync::Arc;
use pyo3::{pyclass, pymethods, pymodule, PyResult, Python};
use pyo3::prelude::PyModule;
use pyo3_tch::{PyTensor, wrap_tch_err};
use crate::config::config::Config;
use crate::server::server::Role::_Worker;

const REDUCE_OP_SUM: &str = "reduce_op_sum";

#[pyclass]
struct IncHandle {
    conf: Config,
}

#[pymethods]
impl IncHandle {
    #[new]
    fn new() -> Self {
        println!("Now we have a handle to call rust functions.");
        Self { conf: Config::new(false, 0, 0) }
    }

    fn init_process_group(&mut self, rank: usize, world_size: usize) -> PyResult<()> {
        println!("try start_udp_service_tokio");
        self.conf = Config::new(false, rank, world_size);
        pyo3_asyncio::tokio::get_runtime().block_on(async {
            let mut s = self.conf.server.as_mut().unwrap().lock().await;
            s.set_role(_Worker);
            s.config_etcd().await;
            s.register_in_etcd().await;
            s.config_peers().await;
            s.config_agg_for_worker().await;
        });
        Ok(())
    }

    fn _all_reduce(&self, tensor: PyTensor, op: String) -> PyResult<PyTensor> {
        let mut tensor = Arc::new(tensor.0);
        if op == REDUCE_OP_SUM {
            let _tensor = tensor.clone();
            pyo3_asyncio::tokio::get_runtime().block_on(async move {
                self.conf.server.clone().unwrap().lock().await.all_reduce_sum(_tensor).await;
            });
            return Ok(PyTensor(Arc::try_unwrap(tensor).unwrap()));
        }
        panic!("impl it");
    }

    fn add_one(&self, tensor: PyTensor) -> PyResult<PyTensor> {
        let tensor = tensor.f_add_scalar(1.0).map_err(wrap_tch_err)?;
        Ok(PyTensor(tensor))
    }

    fn init_agg(&self, world_size: usize) -> PyResult<()> {
        pyo3_asyncio::tokio::get_runtime().block_on(async {
            let mut conf = Config::new_agg(0, world_size);
            {
                let shard_s = conf.server.unwrap();
                let mut s = shard_s.lock().await;
                s.config_etcd().await;
                s.register_in_etcd().await;
                s.config_workers_for_agg().await;
                s.start_udp_service_tokio().await;
            }
        });
        Ok(())
    }
}

#[pymodule]
fn deep_inc(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IncHandle>()?;
    Ok(())
}