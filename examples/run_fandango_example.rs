use std::ffi::CString;

use libafl_fandango_dirty_hack::fandango::FandangoPythonModule;

fn main() -> Result<(), String> {
    let project_root_dir = env!("CARGO_MANIFEST_DIR");
    let fandango =  FandangoPythonModule::new(
        &format!("{}/run_fandango.py", project_root_dir),
        &CString::new("run_fandango.py").unwrap(),
        &CString::new("run_fandango").unwrap(),
        "even_numbers.fan",
    ).map_err(|e| {
        format!(
            "You may need to set the PYTHONPATH environment variable to the path of the Python interpreter, e.g. `export PYTHONPATH=$(echo .venv/lib/python*/site-packages)`. Underlying error: {}",
            e
        )
    })?;

    for _ in 0..10000 {
        fandango.next_input().unwrap();
    }

    Ok(())
}
