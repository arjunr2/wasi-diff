use super::{Error, ExecLog, compute_hash};
use log::debug;

use wasmtime::*;
use wasmtime_wasi::preview1;
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};

struct WasmtimeCtx {
    wasi: preview1::WasiP1Ctx,
    log: ExecLog,
}

fn snapshot(mut caller: Caller<'_, WasmtimeCtx>, num_bytes: i32) {
    let target = format!("{}::snapshot", module_path!());
    debug!(target: &target, "Snapshot function called from module: {:?}", num_bytes);

    // let export = caller.get_export("memory").unwrap();
    let (mem_slice, data) = caller
        .get_export("memory")
        .unwrap()
        .into_memory()
        .unwrap()
        .data_and_store_mut(caller.as_context_mut());

    // Compute memory segment hash
    compute_hash(&mut data.log, mem_slice);
}

pub fn dispatch(command: &Vec<String>) -> Result<ExecLog, Box<dyn Error>> {
    let file = &command[0];
    let args = command;

    let engine = Engine::default();
    let module = Module::from_file(&engine, file)?;

    let mut linker = Linker::new(&engine);
    // Instrumentation function
    linker.func_wrap(
        "env",
        "snapshot",
        |caller: Caller<'_, WasmtimeCtx>, num_bytes: i32| {
            snapshot(caller, num_bytes);
        },
    )?;

    // WASI P1 linking
    preview1::add_to_linker_sync(&mut linker, |t: &mut WasmtimeCtx| &mut t.wasi)?;
    let pre = linker.instantiate_pre(&module)?;

    // Context for WASI and instrumentation
    let context = WasmtimeCtx {
        wasi: WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_env()
            .args(&args)
            .preopened_dir(
                "samples/data",
                "samples/data",
                DirPerms::all(),
                FilePerms::all(),
            )?
            .build_p1(),
        log: ExecLog { hash: None },
    };

    let mut store = Store::new(&engine, context);

    let instance = pre.instantiate(&mut store)?;

    debug!("Executing module...");
    let run = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    let _result = run.call(&mut store, ());
    let _exit_code = 0;

    let exec = &store.data().log;

    Ok(*exec)
}
// Implement xxHash
