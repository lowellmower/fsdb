#![allow(dead_code)]
extern crate fsdb;

use fsdb::*;

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct Single(A);

#[apply_fsdb]
#[derive(Debug, JSONable)]
struct Wrapper {
    string_field: String,
    u8_field: u8,
    // TODO:
    // u16_field: u16,
    // u32_field: u32,
    // u64_field: u64,
}

impl Wrapper {
    fn new() -> Wrapper {
        Wrapper { 
            string_field: "string_field".to_string(), 
            u8_field: 8,
        }
    }
}


#[test]
fn a_struct_can_print_as_json() {
    let wj = 
r#"{
  "string_field": "string_field",
  "u8_field": 8
}"#;

    let w: Wrapper = Wrapper::new();
    assert_eq!(w.fsdb_json(), wj);
}
