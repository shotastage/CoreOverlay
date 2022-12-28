
#[repr(C)]
struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log(msg: String) {
        println!("{}", msg);
    }

    pub fn warning(msg: String) {
        eprintln!("{}", msg);
    }

    pub fn error(msg: String) {
        eprintln!("{}", msg);
    }
}


#[no_mangle]
pub extern "C" fn logger_log(msg: *const c_char) {
    let msg = unsafe { CStr::from_ptr(msg) };
    let msg = msg.to_str().unwrap();
    Logger::log(msg.to_string());
}
