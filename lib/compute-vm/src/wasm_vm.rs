use anyhow::{anyhow, Result};
use std::path::Path;
use wasmer::{Engine, Imports, Instance, Memory, Module, Store, TypedFunction, WasmPtr};

/// WasmRunner is a struct for managing the execution of .wasm files
pub struct WasmRunner {
    store: Store,
    instance: Instance,
}
