use std::time::Duration;
use pyo3::{pyclass, pymethods, pymodule, PyResult, Python};
use pyo3::prelude::PyModule;

use crate::config::config::Config;

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
        let conf = Config::new(true, rank, world_size);
        println!("try start_udp_service_tokio");

        pyo3_asyncio::tokio::get_runtime().spawn(async move {
            conf.server.start_udp_service_tokio().await;
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