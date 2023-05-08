#[no_mangle]
pub extern "C" fn overlay_ping(listen_on: *const libc::c_char) {
    let listen_on = unsafe { std::ffi::CStr::from_ptr(listen_on).to_string_lossy().into_owned() };
    println!("OVERLAY_PING: {}", listen_on);
}
