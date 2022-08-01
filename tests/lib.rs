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
fn test_instance_normal() {
    let loaded = load_resource(env!("MAA_LIB_PATH")).unwrap();
    if loaded {
        let mut ptr = create();
        assert!(!ptr.is_null(), "instance created");
        destroy(&mut ptr);
        assert!(ptr.is_null(), "instance destroyed");
    } else {
        unreachable!()
    }
}
