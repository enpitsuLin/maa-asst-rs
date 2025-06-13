#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_attributes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::raw::MaaCore;
    use std::{ffi::CStr, path::Path};

    fn get_test_lib() -> MaaCore {
        let path = Path::new(env!("MAA_LIB_PATH")).join(
            #[cfg(target_os = "macos")]
            "libMaaCore.dylib",
            #[cfg(target_os = "windows")]
            "MaaCore.dll",
            #[cfg(target_os = "linux")]
            "libMaaCore.so",
        );
        unsafe { MaaCore::new(path).unwrap() }
    }

    #[test]
    fn test_asst_get_null_size() {
        unsafe {
            let lib = get_test_lib();
            let ret = lib.AsstGetNullSize();
            // -1 in cpp, which becomes u64::MAX when cast to unsigned
            let expected = u64::MAX;
            assert_eq!(ret, expected);
        }
    }

    #[test]
    fn test_asst_get_version() {
        unsafe {
            let lib = get_test_lib();
            let version_ptr = lib.AsstGetVersion();
            let version = CStr::from_ptr(version_ptr).to_str().unwrap_or("unknown");
            assert_ne!(version, "unknown");
        }
    }
}
