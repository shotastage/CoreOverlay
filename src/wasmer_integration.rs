use wasmer::{Store, Module, Instance, Value, imports};
use std::ffi::CStr;
use std::os::raw::c_char;



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


fn wasm_exec() -> anyhow::Result<()> {
    let module_wat = r#"
    (module
      (type $t0 (func (param i32) (result i32)))
      (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        get_local $p0
        i32.const 1
        i32.add))
    "#;

    let mut store = Store::default();
    let module = Module::new(&store, &module_wat)?;
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&mut store, &[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));

    Ok(())
}

#[test]
fn test_hello_world() -> anyhow::Result<()> {
    wasm_exec()
}

#[no_mangle]
pub extern "C" fn load_wasm_text_module(wasm_text: *const c_char, main_fn: *const c_char) {
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
pub extern "C" fn load_wasm_module() {
    println!("Loading wasm module");
}

#[no_mangle]
pub extern "C" fn wasm_test() -> i64 {
    wasm_exec().unwrap();
    return 42;
}
