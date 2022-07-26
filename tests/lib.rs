extern crate maa;

use maa::*;

#[test]
fn test_load_resource() {
    let loaded = load_resource("/some/dir/does/exist").unwrap();
    assert!(!loaded);

    let loaded = load_resource(env!("MAA_LIB_PATH")).unwrap();
    assert!(loaded);
}

#[test]
fn test_instance() {
    let loaded = load_resource(env!("MAA_LIB_PATH")).unwrap();
    if loaded {
        let ptr = create();
        destroy(&ptr);
        assert!(ptr.is_null());
    } else {
        unreachable!()
    }
}
