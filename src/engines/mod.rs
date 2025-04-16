use log::debug;
use std::error::Error;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Instrumentation captured data from engine
/// Each engine returns a Log object after execution
#[derive(Debug, Clone, Copy)]
struct ExecLog {
    hash: i64,
    executed: bool,
}

// mod wasmer;
mod wasmtime;

#[derive(Debug, EnumIter)]
enum Engine {
    Wasmtime,
    Wasmer,
}

pub fn dispatch_all(command: &Vec<String>) {
    for engine in Engine::iter() {
        debug!("Engine: {:?}", engine);
        let _ = wasmtime::dispatch(command).unwrap();
    }
}
