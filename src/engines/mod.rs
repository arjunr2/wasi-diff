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

// Engines
mod wasmedge;
mod wasmer;
mod wasmtime;

#[derive(Debug, EnumIter)]
enum Engine {
    Wasmtime,
    Wasmer,
    WasmEdge,
}

pub fn dispatch_all(command: &Vec<String>) {
    for engine in Engine::iter() {
        match engine {
            Engine::Wasmtime => {
                wasmtime::dispatch(command).unwrap();
            }
            Engine::Wasmer => {
                wasmer::dispatch(command).unwrap();
            }
            Engine::WasmEdge => {
                wasmedge::dispatch(command).unwrap();
            }
        }
    }
}
