use wasmer::{Instance, Store};

#[derive(Debug)]
pub struct PkgLoader {
    file_path: String,
    file_hash: String,
    store: Store,
    instance: Instance,
}

impl PkgLoader {
    pub fn new(file_path: String, file_hash: String) -> Self {
        let mut store = Store::default();
        let module = wasmer::Module::new(&store, "(module)").unwrap();
        let import_object = wasmer::imports! {};
        let instance = Instance::new(&mut store, &module, &import_object).unwrap();

        Self {
            file_path,
            file_hash,
            store,
            instance,
        }
    }

    pub fn load(&self) {
        println!("Loading package: {}", self.file_path);
    }

}
