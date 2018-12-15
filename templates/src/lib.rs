extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", to_hash_digest(name)));
}

fn to_hash_digest(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    let b = s.as_bytes();
    hasher.write(b);
    hasher.finish()
}

#[test]
fn test_to_hash_digest() {
    assert_eq!(to_hash_digest("World!"), 531153381967649867);
}
