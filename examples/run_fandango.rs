use clap::Parser;
use libafl_fandango_dirty_hack::fandango::{FandangoPythonModule, FandangoPythonModuleInitError};

#[derive(Parser)]
#[command(name = "run_fandango")]
#[command(about = "Run the fandango interface in Python")]
struct Args {
    #[arg(short, long)]
    python_interface_path: String,
    #[arg(short, long)]
    fandango_file: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let fandango = match FandangoPythonModule::new(&args.python_interface_path, &args.fandango_file)
    {
        Ok(fandango) => fandango,
        Err(FandangoPythonModuleInitError::PyErr(e)) => {
            return Err(format!(
                "You may need to set the PYTHONPATH environment variable to the path of the Python interpreter, e.g. `export PYTHONPATH=$(echo .venv/lib/python*/site-packages)`. Underlying error: {:?}",
                e
            ));
        }
        Err(e) => {
            return Err(format!("Error: {:?}", e));
        }
    };

    for _ in 0..10 {
        println!(
            "{}",
            String::from_utf8(fandango.next_input().unwrap()).unwrap()
        );
    }

    Ok(())
}
