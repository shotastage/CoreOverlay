#[no_mangle]
pub extern "C" fn arch() -> *const c_char {
    let arch = env::consts::ARCH;
    let arch = CString::new(arch).unwrap();
    arch.into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = unsafe { CStr::from_ptr(arch()) };
        let result = result.to_str().unwrap();
        assert_eq!(result, "x86_64");
    }
}
