use std::path::PathBuf;

use clap::Parser;
use libafl::{
    corpus::{InMemoryCorpus, OnDiskCorpus},
    events::{EventConfig, Launcher},
    executors::{ExitKind, InProcessExecutor},
    feedbacks::CrashFeedback,
    fuzzer::{Evaluator, StdFuzzer},
    generators::Generator as _,
    inputs::{BytesInput, HasTargetBytes},
    monitors::MultiMonitor,
    schedulers::QueueScheduler,
    state::StdState,
};
use libafl_bolts::{
    Error,
    core_affinity::Cores,
    current_nanos,
    rands::StdRand,
    shmem::{ShMemProvider, StdShMemProvider},
};
use libafl_fandango_pyo3::{
    fandango::{FandangoPythonModule, FandangoPythonModuleInitError},
    libafl::FandangoGenerator,
};

#[derive(Parser)]
#[command(name = "run_fandango")]
#[command(about = "Run the fandango interface in Python")]
struct Args {
    #[arg(short, long, default_value = "examples/run_fandango.py")]
    python_interface_path: String,
    #[arg(short, long, default_value = "examples/even_numbers.fan")]
    fandango_file: String,
    #[arg(short, long, value_parser = Cores::from_cmdline, default_value = "all")]
    cores: Cores,
}

static VIOLENT_CRASH: bool = true;

fn crash() -> ExitKind {
    if VIOLENT_CRASH {
        panic!("Violent crash");
    } else {
        ExitKind::Crash
    }
}

pub fn main() -> Result<(), String> {
    env_logger::init();

    let args = Args::parse();

    let monitor = MultiMonitor::new(|s| println!("{s}"));

    let shmem_provider = StdShMemProvider::new().expect("Failed to init shared memory");

    // Generate one Generator to ensure the interpreter is ready
    if let Err(FandangoPythonModuleInitError::PyErr(e)) =
        FandangoPythonModule::new(&args.python_interface_path, &args.fandango_file, &[])
    {
        return Err(format!(
            "You may need to set the PYTHONPATH environment variable to the path of the Python interpreter, e.g. `export PYTHONPATH=$(echo .venv/lib/python*/site-packages)`. Underlying error: {:?}",
            e
        ));
    }

    let mut run_client = |state: Option<_>, mut restarting_mgr, _client_description| {
        log::info!("Running client");

        let mut generator = FandangoGenerator::new(
            FandangoPythonModule::new(&args.python_interface_path, &args.fandango_file, &[])
                .unwrap(),
        );

        let mut objective = CrashFeedback::new();

        let mut state = state.unwrap_or_else(|| {
            StdState::new(
                StdRand::with_seed(current_nanos()),
                InMemoryCorpus::new(),
                OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
                &mut (),
                &mut objective,
            )
            .unwrap()
        });

        let mut fuzzer = StdFuzzer::new(QueueScheduler::new(), (), objective);

        let mut harness = |input: &BytesInput| {
            let target = input.target_bytes().to_vec();

            let number = match String::from_utf8(target) {
                Ok(number) => number,
                Err(_) => return crash(),
            };

            let number = match number.parse::<u128>() {
                Ok(number) => number,
                Err(_) => return crash(),
            };

            if number % 2 == 0 {
                ExitKind::Ok
            } else {
                ExitKind::Crash
            }
        };

        let mut executor = InProcessExecutor::new(
            &mut harness,
            (),
            &mut fuzzer,
            &mut state,
            &mut restarting_mgr,
        )
        .expect("Failed to create the Executor");

        loop {
            let input = match generator.generate(&mut fuzzer) {
                Ok(input) => input,
                Err(e) => {
                    println!("Error generating input: {e:?}");
                    continue;
                }
            };
            fuzzer
                .evaluate_input(&mut state, &mut executor, &mut restarting_mgr, &input)
                .unwrap();
        }
    };

    match Launcher::builder()
        .shmem_provider(shmem_provider)
        .configuration(EventConfig::from_name("default"))
        .monitor(monitor)
        .run_client(&mut run_client)
        .cores(&args.cores)
        .broker_port(1337)
        .build()
        .launch()
    {
        Ok(()) => (),
        Err(Error::ShuttingDown) => println!("Fuzzing stopped by user. Good bye."),
        Err(err) => return Err(format!("Failed to run launcher: {err:?}")),
    }

    Ok(())
}
