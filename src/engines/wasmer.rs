use super::{Error, ExecLog};
use log::debug;
use std::fs;
use wasmer::{Extern, Function, FunctionEnv, FunctionEnvMut, Module, Store};
use wasmer_wasix::WasiEnv;

fn snapshot(mut env: FunctionEnvMut<ExecLog>, num_bytes: i32) {
    let target = format!("{}::snapshot", module_path!());
    debug!(target: &target, "Snapshot function called from module: {:?}", num_bytes);
    // env.data_mut().executed = true;
}

pub fn dispatch(command: &Vec<String>) -> Result<ExecLog, Box<dyn Error>> {
    let filepath = &command[0];
    let args = &command[1..];

    // Read file
    let bin = fs::read(filepath)?;

    let mut store = Store::default();
    let module = Module::new(&mut store, bin)?;

    // Context for instrumentation
    let context = FunctionEnv::new(&mut store, ExecLog { hash: None });

    debug!("Executing module...");
    let _ = WasiEnv::builder(filepath)
        .args(args)
        .preopen_dir("samples/data")?
        .import(
            "env",
            "snapshot",
            Extern::Function(Function::new_typed_with_env(&mut store, &context, snapshot)),
        )
        .run_with_store(module, &mut store)?;

    Ok(context.as_ref(&mut store).clone())
}
