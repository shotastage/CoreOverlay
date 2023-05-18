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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::env;

    #[test]
    fn test_ovr_arch() {
        let arch_ptr = ovr_arch();
        let arch_cstr = unsafe { CStr::from_ptr(arch_ptr) };
        let arch_str = arch_cstr.to_str().unwrap();

        assert_eq!(env::consts::ARCH, arch_str);
        unsafe { CString::from_raw(arch_ptr as *mut c_char) };
    }

    #[test]
    fn test_ovr_os() {
        let os_ptr = ovr_os();
        let os_cstr = unsafe { CStr::from_ptr(os_ptr) };
        let os_str = os_cstr.to_str().unwrap();

        assert_eq!(env::consts::OS, os_str);
        unsafe { CString::from_raw(os_ptr as *mut c_char) };
    }

    #[test]
    fn test_ovr_user_agent() {
        let user_agent_ptr = ovr_user_agent();
        let user_agent_cstr = unsafe { CStr::from_ptr(user_agent_ptr) };
        let user_agent_str = user_agent_cstr.to_str().unwrap();

        assert_eq!("OVERLAY/0.1.0 COREOVERLAY/0.1.0 DHT/Kademlia WASMER/3.1.0", user_agent_str);
        unsafe { CString::from_raw(user_agent_ptr as *mut c_char) };
    }
}
