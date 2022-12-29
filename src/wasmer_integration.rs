use wasmer::{Store, Module, Instance, Value, imports};

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
pub extern "C" fn wasm_test() -> i64 {
    wasm_exec().unwrap();
    return 42;
}
