use crate::Serialize;


#[test]
fn string_test() {
    let string = "Hello, World!!!".to_string();
    let bytes = string.clone().serialize();
    let found_string = String::deserialize(&bytes, 0).unwrap();
    assert_eq!(string, found_string);
}

use crate as serialr;
#[derive(Serialize, Clone, Debug, Copy, PartialEq)]
pub struct Test {
    x: u64,
    y: u64,
}
#[test]
fn derive_test() {
    let string = Test {x: 345, y: 564746324436543543};
    let bytes = string.serialize();
    let found_string = Test::deserialize(&bytes, 0).unwrap();
    assert_eq!(string, found_string);
}
#[test]
fn enum_derive_test() {
    let enum1 = TestEnum::Variant1(34, "Test 1".to_string());
    let bytes = enum1.clone().serialize();
    let found_enum = TestEnum::deserialize(&bytes, 0).unwrap();
    assert_eq!(enum1, found_enum);
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum TestEnum {
    Variant0(u64, String),
    Variant1(u8, String),
}
