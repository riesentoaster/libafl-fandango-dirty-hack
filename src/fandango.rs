use std::ffi::{CStr, CString};

use pyo3::prelude::*;

pub struct FandangoPythonModule {
    module: Py<PyModule>,
    generator: Py<PyAny>,
}

impl FandangoPythonModule {
    pub fn new(
        path: &str,
        file_name: &CStr,
        module_name: &CStr,
        fandango_file: &str,
    ) -> Result<Self, PyErr> {
        let code = CString::new(std::fs::read_to_string(path)?)?;
        Python::with_gil(|py| {
            let module = PyModule::from_code(py, &code, file_name, module_name)?;
            let module: Py<PyModule> = module.into();

            let generator = module.getattr(py, "setup")?.call1(py, (fandango_file,))?;

            Ok(Self { module, generator })
        })
    }

    pub fn next_input(&self) -> Result<Vec<u8>, PyErr> {
        Python::with_gil(|py| {
            let generator = self.generator.clone_ref(py);
            let input = self
                .module
                .getattr(py, "next_input")?
                .call1(py, (generator,))?
                .extract::<Vec<u8>>(py)?;
            Ok(input)
        })
    }
}
