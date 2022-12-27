struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log(msg: String) -> String {
        println!(msg);
    }
}


#[no_mangle]
pub extern "C" fn log() -> *const c_char {
    let log = "Hello, world!";
    let log = CString::new(log).unwrap();
    log.into_raw()
}
