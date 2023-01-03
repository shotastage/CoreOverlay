use std::env;
use std::ffi::c_char;
use std::ffi::CString;


#[no_mangle]
pub extern "C" fn ovr_arch() -> *const c_char {
    let arch = env::consts::ARCH;
    let arch = CString::new(arch).unwrap();
    
    return arch.into_raw()
}
