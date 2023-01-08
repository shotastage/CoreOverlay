use std::ffi::{CString, c_char};
use sha1::{Sha1, Digest};


fn gen_sha1(data: &str) -> String {
    // create a Sha1 object
    let mut hasher = Sha1::new();

    // process input message
    hasher.update(data);

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();
    //assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
    return result;
}

#[no_mangle]
pub extern "C" fn sha1(data: &str) -> *const c_char {

    let digest = gen_sha1(data);

    let c_str =  CString::new(digest).unwrap();

    return c_str.into_raw();
}
