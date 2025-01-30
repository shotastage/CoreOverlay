mod pkg_loader;
pub mod wasm_vm;

use anyhow::{anyhow, Result};
use std::path::Path;
use wasmer::{Engine, Imports, Instance, Memory, Module, Store, TypedFunction, WasmPtr};

/// WasmRunner is a struct for managing the execution of .wasm files
pub struct WasmRunner {
    store: Store,
    instance: Instance,
}

impl WasmRunner {
    /// Creates a new WasmRunner instance
    pub fn new<P: AsRef<Path>>(wasm_path: P) -> Result<Self> {
        let engine = Engine::default();
        let mut store = Store::new(engine);
        let wasm_bytes = std::fs::read(wasm_path)?;
        let module = Module::new(&store, wasm_bytes)?;
        let imports = Imports::new();
        let instance = Instance::new(&mut store, &module, &imports)?;
        Ok(Self { store, instance })
    }

    /// Executes the specified function
    pub fn call_function(
        &mut self,
        func_name: &str,
        params: &[wasmer::Value],
    ) -> Result<Box<[wasmer::Value]>> {
        let func = self
            .instance
            .exports
            .get_function(func_name)
            .map_err(|_| anyhow!("Function {} not found", func_name))?;
        Ok(func.call(&mut self.store, params)?)
    }

    /// Retrieves a typed function
    pub fn get_typed_function<Params, Results>(
        &mut self,
        func_name: &str,
    ) -> Result<TypedFunction<Params, Results>>
    where
        Params: wasmer::WasmTypeList,
        Results: wasmer::WasmTypeList,
    {
        let func = self
            .instance
            .exports
            .get_typed_function(&mut self.store, func_name)
            .map_err(|_| anyhow!("Typed function {} not found", func_name))?;
        Ok(func)
    }

    /// Helper function to access memory
    pub fn get_memory(&self) -> Result<Memory> {
        self.instance
            .exports
            .get_memory("memory")
            .map(|mem| (*mem).clone())
            .map_err(|_| anyhow!("Memory not found in module"))
    }

    /// Helper function to write values to memory
    pub fn write_memory(&mut self, offset: u32, data: &[u8]) -> Result<()> {
        let memory = self.get_memory()?;
        let view = memory.view(&self.store);
        view.write(offset as u64, data)?;
        Ok(())
    }

    /// Helper function to read values from memory
    pub fn read_memory(&self, offset: u32, length: u32) -> Result<Vec<u8>> {
        let memory = self.get_memory()?;
        let view = memory.view(&self.store);
        let mut buffer = vec![0u8; length as usize];
        view.read(offset as u64, &mut buffer)?;
        Ok(buffer)
    }

    /// Helper function to read a string from memory using WasmPtr
    pub fn read_string(&self, ptr: WasmPtr<u8>, len: u32) -> Result<String> {
        let memory = self.get_memory()?;
        let view = memory.view(&self.store);
        let data = ptr.read_utf8_string(&view, len)?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasmer::Value;

    #[test]
    fn test_basic_wasm_execution() -> Result<()> {
        let wasm_path = "path/to/test.wasm";
        let mut runner = WasmRunner::new(wasm_path)?;
        let result = runner.call_function("add", &[Value::I32(1), Value::I32(2)])?;
        assert_eq!(result[0], Value::I32(3));
        Ok(())
    }

    #[test]
    fn test_typed_function() -> Result<()> {
        let wasm_path = "path/to/test.wasm";
        let mut runner = WasmRunner::new(wasm_path)?;
        let add_func: TypedFunction<(i32, i32), i32> = runner.get_typed_function("add")?;
        let result = add_func.call(&mut runner.store, 1, 2)?;
        assert_eq!(result, 3);
        Ok(())
    }
}
