use log::{error, info, warn};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use xxhash_rust::xxh3::xxh3_128;

/// Instrumentation captured data from engine
/// Each engine returns a Log object after execution
#[derive(Debug, Clone, Copy)]
struct ExecLog {
    hash: Option<u128>,
}

fn compute_hash(data: &mut ExecLog, input: &[u8]) {
    // Hash the data
    data.hash = Some(xxh3_128(input));
}

// Engines
mod wasmedge;
mod wasmer;
mod wasmtime;

#[derive(Debug, EnumIter, Eq, PartialEq, Hash)]
enum Engine {
    Wasmtime,
    Wasmer,
    WasmEdge,
}

pub fn dispatch_all(command: &Vec<String>) {
    let results: HashMap<Engine, ExecLog> = Engine::iter()
        .map(|e| {
            let x = match e {
                Engine::Wasmtime => wasmtime::dispatch(command),
                Engine::WasmEdge => wasmedge::dispatch(command),
                _ => {
                    warn!("Unsupported engine: {:?}", e);
                    Ok(ExecLog { hash: None })
                }
            };
            let x = match x {
                Ok(v) => {
                    info!("{:?} executed successfully", e);
                    v
                }
                Err(err) => {
                    error!("{:?} -- {:?}", e, err);
                    ExecLog { hash: None }
                }
            };
            (e, x)
        })
        .collect();

    let mut final_results: HashMap<u128, HashSet<&Engine>> = HashMap::new();
    for (engine, log) in results.iter() {
        if let Some(hash) = log.hash {
            final_results
                .entry(hash)
                .or_insert_with(HashSet::new)
                .insert(engine);
        }
    }
    info!("Results: {:?}", final_results);
    info!("Num Unique: {:?}", final_results.len());
}
