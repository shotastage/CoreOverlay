struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log(&self) -> String {
        "Hello, world!".to_string();
        println!("Hello, world!");
    }
}


#[no_mangle]
pub extern "C" fn log() -> *const c_char {
    let log = "Hello, world!";
    let log = CString::new(log).unwrap();
    log.into_raw()
}
