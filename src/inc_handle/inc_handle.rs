use pyo3::{pyclass, pymethods, pymodule, PyResult, Python};
use pyo3::prelude::PyModule;
use serde::de::Unexpected::Option;

use crate::config::config::Config;
use crate::server::server::Role::_Worker;

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

        pyo3_asyncio::tokio::get_runtime().spawn(async move {
            let s = conf.server.as_mut().unwrap();
            s.set_role(_Worker);
            s.config_etcd().await;
            s.register_in_etcd().await;
            s.config_peers().await;
            s.config_agg().await;
            s.start_udp_service_tokio().await;
        });
        Ok(())
    }

    fn all_reduce(&self) -> PyResult<()> {
        // 实现
        Ok(())
    }
}

#[pymodule]
fn deep_inc(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IncHandle>()?;
    Ok(())
}