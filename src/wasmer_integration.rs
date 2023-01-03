use anyhow::Ok;
use wasmer::{imports, EngineBuilder, Instance, Module, Store, Value};
use std::ffi::CStr;
use std::os::raw::{c_char, c_uchar};
use core::slice;

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


// C functions that can be called from the C code
#[no_mangle]
pub extern "C" fn c_exec_wasm_text_module(wasm_text: *const c_char, main_fn: *const c_char) {
    println!("Loading wasm module");

    // Convert the C string to a Rust string
    let c_wasm_text: &CStr = unsafe { CStr::from_ptr(wasm_text) };
    let wasm_text_: &str = c_wasm_text.to_str().unwrap();

    let c_main_fn: &CStr = unsafe { CStr::from_ptr(main_fn) };
    let main_fn_: &str = c_main_fn.to_str().unwrap();

    // Execute the wasm text module
    wasm_text_exec(wasm_text_, main_fn_).unwrap();
    return;
}

#[no_mangle]
pub extern "C" fn c_exec_wasm_native_module(data: *const c_uchar, data_length: usize) {
    println!("Loading wasm module");

    let c_data = unsafe {slice::from_raw_parts(data, data_length)};
    let r_data: Vec<u8> = Vec::from(c_data);

    wasm_native_exec(&r_data).unwrap();
}
