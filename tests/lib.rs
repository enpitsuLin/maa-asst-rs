extern crate maa;

use maa::*;

#[test]
fn test_load_resource() {
    let loaded = load_resource("/some/dir/does/exist").unwrap();
    assert!(!loaded)
}
