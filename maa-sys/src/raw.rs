#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_attributes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::raw;

    #[test]
    fn test_binding_version() {
        unsafe {
            let ret = raw::AsstGetNullSize();
            // -1 in cpp, which becomes u64::MAX when cast to unsigned
            let expected = u64::MAX;
            assert_eq!(ret, expected);
        }
    }
}
