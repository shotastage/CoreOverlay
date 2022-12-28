use std::ffi::CString;


#[repr(C)]
struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log(msg: String) {
        println!("{}", msg);
    }
}


#[no_mangle]
pub extern "C" fn log(msg: CString) {
    print!("{}", msg.into_string().unwrap());
    
}
