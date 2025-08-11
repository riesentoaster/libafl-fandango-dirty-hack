use std::path::PathBuf;

use clap::Parser;
use libafl::Evaluator;
use libafl::generators::Generator as _;
use libafl::monitors::SimpleMonitor;
use libafl::{
    corpus::{InMemoryCorpus, OnDiskCorpus},
    events::SimpleEventManager,
    executors::{ExitKind, InProcessExecutor},
    feedbacks::CrashFeedback,
    fuzzer::StdFuzzer,
    inputs::{BytesInput, HasTargetBytes},
    schedulers::QueueScheduler,
    state::StdState,
};
use libafl_bolts::{current_nanos, rands::StdRand};
use libafl_fandango_dirty_hack::{fandango::FandangoPythonModule, libafl::FandangoGenerator};

#[derive(Parser)]
#[command(name = "run_fandango")]
#[command(about = "Run the fandango interface in Python")]
struct Args {
    python_interface_path: String,
    fandango_file: String,
}

static VIOLENT_CRASH: bool = false;

pub fn main() {
    env_logger::init();

    let args = Args::parse();

    let mut harness = |input: &BytesInput| {
        let target = input.target_bytes().to_vec();

        let number = if VIOLENT_CRASH {
            String::from_utf8(target).unwrap()
        } else {
            match String::from_utf8(target.to_vec()) {
                Ok(number) => number,
                Err(_) => return ExitKind::Crash,
            }
        };

        let number = if VIOLENT_CRASH {
            number.parse::<u128>().unwrap()
        } else {
            match number.parse::<u128>() {
                Ok(number) => number,
                Err(_) => return ExitKind::Crash,
            }
        };

        if number % 2 == 0 {
            ExitKind::Ok
        } else {
            ExitKind::Crash
        }
    };

    let mut objective = CrashFeedback::new();

    let mut state = StdState::new(
        StdRand::with_seed(current_nanos()),
        InMemoryCorpus::new(),
        OnDiskCorpus::new(PathBuf::from("./crashes")).unwrap(),
        &mut (),
        &mut objective,
    )
    .unwrap();

    let mut fuzzer = StdFuzzer::new(QueueScheduler::new(), (), objective);

    let mon = SimpleMonitor::new(|s| println!("{s}"));
    let mut mgr = SimpleEventManager::new(mon);

    let mut executor = InProcessExecutor::new(&mut harness, (), &mut fuzzer, &mut state, &mut mgr)
        .expect("Failed to create the Executor");

    let mut generator = FandangoGenerator::new(
        FandangoPythonModule::new(&args.python_interface_path, &args.fandango_file).unwrap(),
    );

    loop {
        let input = generator.generate(&mut fuzzer).unwrap();
        fuzzer
            .evaluate_input(&mut state, &mut executor, &mut mgr, &input)
            .unwrap();
    }
}
