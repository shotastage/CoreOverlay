use std::path::Path;
use wasmer::{
    Engine, Instance, Module, Store, Imports, Function, TypedFunction,
    WasmPtr, Memory
};
use anyhow::{Result, anyhow};

/// WasmRunner は .wasm ファイルの実行を管理するための構造体です
pub struct WasmRunner {
    store: Store,
    instance: Instance,
}

impl WasmRunner {
    /// 新しい WasmRunner インスタンスを作成します
    pub fn new<P: AsRef<Path>>(wasm_path: P) -> Result<Self> {
        let engine = Engine::default();
        let mut store = Store::new(&engine);
        let wasm_bytes = std::fs::read(wasm_path)?;
        let module = Module::new(&store, wasm_bytes)?;
        let imports = Imports::new();
        let instance = Instance::new(&mut store, &module, &imports)?;
        Ok(Self { store, instance })
    }

    /// 指定された関数を実行します
    pub fn call_function(&mut self, func_name: &str, params: &[wasmer::Value]) -> Result<Box<[wasmer::Value]>> {
        let func = self.instance
            .exports
            .get_function(func_name)
            .map_err(|_| anyhow!("Function {} not found", func_name))?;
        Ok(func.call(&mut self.store, params)?)
    }

    /// 型付き関数を取得します
    pub fn get_typed_function<Params, Results>(
        &mut self,
        func_name: &str,
    ) -> Result<TypedFunction<Params, Results>> {
        let func = self.instance
            .exports
            .get_typed_function(&mut self.store, func_name)
            .map_err(|_| anyhow!("Typed function {} not found", func_name))?;
        Ok(func)
    }

    /// メモリにアクセスするためのヘルパー関数
    pub fn get_memory(&self) -> Result<Memory> {
        self.instance
            .exports
            .get_memory("memory")
            .map_err(|_| anyhow!("Memory not found in module"))
    }

    /// メモリに値を書き込むヘルパー関数
    pub fn write_memory(&mut self, offset: u32, data: &[u8]) -> Result<()> {
        let memory = self.get_memory()?;
        let view = memory.view();
        view.write(offset as usize, data)?;
        Ok(())
    }

    /// メモリから値を読み込むヘルパー関数
    pub fn read_memory(&self, offset: u32, length: u32) -> Result<Vec<u8>> {
        let memory = self.get_memory()?;
        let view = memory.view();
        let mut buffer = vec![0u8; length as usize];
        view.read(offset as usize, &mut buffer)?;
        Ok(buffer)
    }

    /// WasmPtrを使用してメモリから文字列を読み込むヘルパー関数
    pub fn read_string(&self, ptr: WasmPtr<u8>, len: u32) -> Result<String> {
        let memory = self.get_memory()?;
        let view = memory.view();
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
