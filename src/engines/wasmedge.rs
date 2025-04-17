use super::{Error, ExecLog, compute_hash};
use log::debug;
use std::collections::HashMap;
use std::slice;

use wasmedge_sdk::{
    AsInstance, CallingFrame, ImportObjectBuilder, Instance, Module, Store, ValType, Vm, WasmValue,
    error::CoreError, params, vm::SyncInst, wasi::WasiModule,
};

fn snapshot(
    data: &mut ExecLog,
    _inst: &mut Instance,
    caller: &mut CallingFrame,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    if input.len() != 1 && input[0].ty() != ValType::I32 {
        return Err(CoreError::Execution(
            wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
        ));
    }

    let num_bytes = input[0].to_i32() as u32;
    let target = format!("{}::snapshot", module_path!());
    debug!(target: &target, "Snapshot function called from module: {:?}", num_bytes);

    unsafe {
        // Compute memory segment hash
        let mem_ptr = caller
            .memory_ref(0)
            .unwrap()
            .data_pointer(0, num_bytes)
            .unwrap();
        let mem_slice = slice::from_raw_parts(mem_ptr, num_bytes as usize);
        compute_hash(data, mem_slice);
    }

    Ok(vec![])
}

pub fn dispatch(command: &Vec<String>) -> Result<ExecLog, Box<dyn Error>> {
    let filepath = &command[0];
    let args = command;

    let module = Module::from_file(None, filepath)?;

    // Context for instrumentation
    let context = ExecLog { hash: None };

    // Construct imports
    let mut import_builder = ImportObjectBuilder::new("env", context)?;
    import_builder.with_func::<i32, ()>("snapshot", snapshot)?;
    let mut import_object = import_builder.build();

    // WASI setup
    let mut wasi_module = WasiModule::create(
        Some(args.iter().map(|x| x.as_str()).collect()),
        None,
        Some(vec!["samples/data"]),
    )
    .unwrap();

    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    instances.insert(import_object.name().unwrap(), &mut import_object);

    let mut vm = Vm::new(Store::new(None, instances)?);
    vm.register_module(None, module)?;

    debug!("Executing module...");
    vm.run_func(None, "_start", params!())?;

    let result = import_object.get_host_data();

    Ok(*result)
}
