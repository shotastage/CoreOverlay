use anyhow::Ok;
use wasmer::{imports, EngineBuilder, Instance, Module, Store, Value};

pub fn wasmer_exec(file_binary: &[u8]) -> anyhow::Result<()> {
    let engine = EngineBuilder::headless();
    let mut store = Store::new(engine);
    let module = Module::from_binary(&store, file_binary)?;

    let import_object = imports! {};

    let instance = Instance::new(&mut store, &module, &import_object)?;

    let sum = instance.exports.get_function("sum")?;
    let _results = sum.call(&mut store, &[Value::I32(1), Value::I32(2)])?;

    Ok(())
}

// Rust function that calls the wasm text module
fn wasm_text_exec(wat_module: &str, main_fn: &str) -> anyhow::Result<()> {
    let mut store = Store::default();
    let module = Module::new(&store, &wat_module)?;
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function(main_fn)?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));

    Ok(())
}

// Rust function that calls the pre-compiled wasm module
fn wasm_native_exec(file_binary: &[u8]) -> anyhow::Result<()> {
    let engine = EngineBuilder::headless();
    let mut store = Store::new(engine);
    let module = Module::from_binary(&store, file_binary)?;

    let import_object = imports! {};

    let instance = Instance::new(&mut store, &module, &import_object)?;

    let sum = instance.exports.get_function("sum")?;
    let _results = sum.call(&mut store, &[Value::I32(1), Value::I32(2)])?;

    Ok(())
}

// Test functions
#[test]
fn test_hello_world() -> anyhow::Result<()> {
    // wasm_exec("hello_world.wasm")
    Ok(())
}
