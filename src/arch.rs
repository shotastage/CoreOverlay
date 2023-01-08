use std::env;
use std::ffi::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn ovr_arch() -> *const c_char {
    let arch = env::consts::ARCH;
    let arch = CString::new(arch).unwrap();

    return arch.into_raw();
}

#[no_mangle]
pub extern "C" fn ovr_os() -> *const c_char {
    let os = env::consts::OS;
    let os = CString::new(os).unwrap();

    return os.into_raw();
}

#[no_mangle]
pub extern "C" fn ovr_user_agent() -> *const c_char {
    let user_agent = "OVERLAY/0.1.0 COREOVERLAY/0.1.0 DHT/Kademlia WASMER/3.1.0";
    let user_agent = CString::new(user_agent).unwrap();

    return user_agent.into_raw();
}
