use wasmer::{Instance, Store};

#[derive(Debug)]
pub struct PkgLoader {
    file_path: String,
    file_hash: String,
    store: Store,
    instance: Instance,
}
