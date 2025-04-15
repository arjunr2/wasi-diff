use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod wasmtime;

/// Interface supported by all Wasm engines
trait EngineT {
    fn new() -> Self;
    // fn load(&self) -> Result<(), String>;
    // fn instantiate(&self) -> Result<(), String>;
    // fn run(&self, args: Vec<String>) -> Result<(), String>;
    // fn hash_state(&self) -> String;
}

#[derive(Debug, EnumIter)]
enum Engine {
    Wasmtime,
    Wasmer,
}

impl EngineT for Engine {
    fn new() -> Self {
        Engine::Wasmtime
    }
}

pub fn engine_variants() {
    for engine in Engine::iter() {
        println!("Engine: {:?}", engine);
    }
    wasmtime::test_method();
}
