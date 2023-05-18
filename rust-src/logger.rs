use std::ffi::c_char;

#[no_mangle]
pub extern "C" fn ovry_info(msg: *const c_char) {
    println!("{:?}", msg);
}

#[no_mangle]
pub extern "C" fn ovry_log(msg: *const c_char) {
    println!("{:?}", msg);
}

#[no_mangle]
pub extern "C" fn ovry_warning(msg: *const c_char) {
    println!("{:?}", msg);
}

#[no_mangle]
pub extern "C" fn ovry_error(msg: *const c_char) {
    println!("{:?}", msg);
}
